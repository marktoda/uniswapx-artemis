use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum KeyStoreError {
    KeyNotFound,
    LockError,
}

impl fmt::Display for KeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyStoreError::KeyNotFound => write!(f, "Key not found"),
            KeyStoreError::LockError => write!(f, "Failed to acquire lock"),
        }
    }
}

impl std::error::Error for KeyStoreError {}

// Private key type to prevent accidental exposure of private keys
// Does not implement Debug
pub struct PrivateKey(String);
impl PrivateKey {
    pub fn new(key: String) -> Self {
        PrivateKey(key)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl Clone for PrivateKey {
    fn clone(&self) -> Self {
        PrivateKey(self.0.clone())
    }
}
impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone)]
pub struct KeyStore {
    keys: Arc<Mutex<HashMap<String, (PrivateKey, bool)>>>, // Public address to (private key, in-use flag) mapping
    notify: Arc<Notify>,
}

impl KeyStore {
    pub fn new() -> Self {
        KeyStore {
            keys: Arc::new(Mutex::new(HashMap::new())),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn add_key(&self, public_address: String, private_key: String) {
        let mut keys = self.keys.lock().await;
        keys.insert(public_address, (PrivateKey::new(private_key), false));
    }

    pub async fn acquire_key(&self) -> Result<(String, PrivateKey), KeyStoreError> {
        loop {
            let mut keys: tokio::sync::MutexGuard<'_, HashMap<String, (PrivateKey, bool)>> =
                self.keys.lock().await;
            if let Some((public_address, (private_key, in_use))) =
                keys.iter_mut().find(|(_, (_, in_use))| !*in_use)
            {
                *in_use = true;
                return Ok((public_address.clone(), private_key.clone()));
            }
            drop(keys); // Release the lock before waiting
            println!("Waiting for key");
            self.notify.notified().await;
        }
    }

    pub async fn release_key(&self, public_address: String) -> Result<(), KeyStoreError> {
        let mut keys = self.keys.lock().await;
        if let Some((_, in_use)) = keys.get_mut(&public_address) {
            *in_use = false;
            self.notify.notify_one();
            Ok(())
        } else {
            Err(KeyStoreError::KeyNotFound)
        }
    }

    pub async fn len(&self) -> usize {
        let keys = self.keys.lock().await;
        keys.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::sync::Notify;

    #[tokio::test]
    async fn test_acquire_key() {
        let mut keys: HashMap<String, (PrivateKey, bool)> = HashMap::new();
        keys.insert(
            "address1".to_string(),
            (PrivateKey::new("private_key1".to_string()), false),
        );
        let keystore = KeyStore {
            keys: Arc::new(Mutex::new(keys)),
            notify: Arc::new(Notify::new()),
        };

        // Test for valid key retrieval
        let (public_key, private_key) = keystore.acquire_key().await.unwrap();
        if private_key != PrivateKey::new("private_key1".to_string())
            || private_key.as_str() != "private_key1"
        {
            panic!("Private key not found");
        }

        // Test that the key is marked as in-use
        let keys = keystore.keys.lock().await;
        let (_, in_use) = keys.get(&public_key).unwrap();
        assert!(*in_use);
    }

    #[tokio::test]
    async fn test_len() {
        let mut keys = HashMap::new();
        keys.insert(
            "address1".to_string(),
            (PrivateKey::new("private_key1".to_string()), true),
        );
        keys.insert(
            "address2".to_string(),
            (PrivateKey::new("private_key2".to_string()), true),
        );
        let keystore = KeyStore {
            keys: Arc::new(Mutex::new(keys)),
            notify: Arc::new(Notify::new()),
        };

        // Test for correct length
        let len = keystore.len().await;
        assert_eq!(len, 2);
    }

    #[tokio::test]
    async fn test_release_nonexistent_key() {
        let keys: HashMap<String, (PrivateKey, bool)> = HashMap::new();
        let keystore = KeyStore {
            keys: Arc::new(Mutex::new(keys)),
            notify: Arc::new(Notify::new()),
        };

        // Attempt to release a key that is not in the store
        let result = keystore
            .release_key("nonexistent_address".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), KeyStoreError::KeyNotFound);
    }
}
