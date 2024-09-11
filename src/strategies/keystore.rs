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
    keys: HashMap<String, Arc<Mutex<(PrivateKey, bool)>>>,
    notify: Arc<Notify>,
}

impl KeyStore {
    pub fn new() -> Self {
        KeyStore {
            keys: HashMap::new(),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn add_key(&mut self, public_address: String, private_key: String) {
        self.keys.insert(
            public_address,
            Arc::new(Mutex::new((PrivateKey::new(private_key), false))),
        );
    }

    pub async fn acquire_key(&self) -> Result<(String, PrivateKey), KeyStoreError> {
        loop {
            for (public_address, key_mutex) in &self.keys {
                let mut key_data = key_mutex.lock().await;
                let (private_key, in_use) = &mut *key_data;
                if !*in_use {
                    *in_use = true;
                    return Ok((public_address.clone(), private_key.clone()));
                }
            }
            self.notify.notified().await;
        }
    }

    pub async fn release_key(&self, public_address: String) -> Result<(), KeyStoreError> {
        if let Some(key_mutex) = self.keys.get(&public_address) {
            let mut key_data = key_mutex.lock().await;
            key_data.1 = false;
            self.notify.notify_one();
            Ok(())
        } else {
            Err(KeyStoreError::KeyNotFound)
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_acquire_key() {
        let mut keystore = KeyStore::new();
        keystore
            .add_key("address1".to_string(), "private_key".to_string())
            .await;
        keystore
            .add_key("address2".to_string(), "private_key".to_string())
            .await;

        // Test for valid key retrieval
        let (public_key, private_key) = keystore.acquire_key().await.unwrap();
        assert_eq!(private_key.as_str(), "private_key");

        // Test that the key is marked as in-use
        let key_data = keystore.keys.get(&public_key).unwrap().lock().await;
        assert!(key_data.1);
        drop(key_data);

        // Test for valid key retrieval of the second key
        let (public_key2, private_key2) = keystore.acquire_key().await.unwrap();
        assert_eq!(private_key2.as_str(), "private_key");
        assert_ne!(public_key, public_key2);

        // Test that the second key is marked as in-use
        let key_data2 = keystore.keys.get(&public_key2).unwrap().lock().await;
        assert!(key_data2.1);
        drop(key_data2);
    }

    #[tokio::test]
    async fn test_len() {
        let mut keystore = KeyStore::new();
        keystore
            .add_key("address1".to_string(), "private_key1".to_string())
            .await;
        keystore
            .add_key("address2".to_string(), "private_key2".to_string())
            .await;

        // Test for correct length
        assert_eq!(keystore.len(), 2);
    }

    #[tokio::test]
    async fn test_release_nonexistent_key() {
        let keystore = KeyStore::new();

        // Attempt to release a key that is not in the store
        let result = keystore
            .release_key("nonexistent_address".to_string())
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), KeyStoreError::KeyNotFound);
    }

    #[tokio::test]
    async fn test_notify_on_key_release() {
        let mut keystore = KeyStore::new();
        keystore.add_key("address1".to_string(), "private_key1".to_string()).await;
        keystore.add_key("address2".to_string(), "private_key2".to_string()).await;

        // Acquire both keys
        let (addr1, _) = keystore.acquire_key().await.unwrap();
        let (addr2, _) = keystore.acquire_key().await.unwrap();

        // Attempt to acquire a key (should wait)
        let acquire_future = tokio::spawn({
            let keystore = keystore.clone();
            async move {
                keystore.acquire_key().await
            }
        });

        // Wait a bit to ensure the acquire_future is waiting
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Release one key
        keystore.release_key(addr1.clone()).await.unwrap();

        // The acquire_future should now complete
        let acquire_result = timeout(Duration::from_secs(1), acquire_future).await;
        assert!(acquire_result.is_ok(), "Acquire operation timed out");
        
        let (acquired_addr, _) = acquire_result.unwrap().unwrap().unwrap();
        assert_eq!(acquired_addr, addr1, "Acquired address should match released address");

        // Clean up
        keystore.release_key(addr2).await.unwrap();
        keystore.release_key(acquired_addr).await.unwrap();
    }
}
