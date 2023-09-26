pub use resolved_order_lib::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod resolved_order_lib {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"DeadlinePassed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidReactor\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static RESOLVEDORDERLIB_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        86,
        96,
        55,
        96,
        11,
        130,
        130,
        130,
        57,
        128,
        81,
        96,
        0,
        26,
        96,
        115,
        20,
        96,
        42,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        0,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        48,
        96,
        0,
        82,
        96,
        115,
        129,
        83,
        130,
        129,
        243,
        254,
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        5,
        16,
        33,
        222,
        219,
        115,
        14,
        109,
        93,
        251,
        183,
        126,
        145,
        26,
        143,
        125,
        15,
        226,
        189,
        4,
        250,
        19,
        135,
        3,
        69,
        187,
        224,
        253,
        127,
        115,
        220,
        153,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static RESOLVEDORDERLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        115,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        48,
        20,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        5,
        16,
        33,
        222,
        219,
        115,
        14,
        109,
        93,
        251,
        183,
        126,
        145,
        26,
        143,
        125,
        15,
        226,
        189,
        4,
        250,
        19,
        135,
        3,
        69,
        187,
        224,
        253,
        127,
        115,
        220,
        153,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static RESOLVEDORDERLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ResolvedOrderLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ResolvedOrderLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ResolvedOrderLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ResolvedOrderLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ResolvedOrderLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ResolvedOrderLib)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ResolvedOrderLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    RESOLVEDORDERLIB_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                RESOLVEDORDERLIB_ABI.clone(),
                RESOLVEDORDERLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ResolvedOrderLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DeadlinePassed` with signature `DeadlinePassed()` and selector `0x70f65caa`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DeadlinePassed", abi = "DeadlinePassed()")]
    pub struct DeadlinePassed;
    ///Custom Error type `InvalidReactor` with signature `InvalidReactor()` and selector `0x4ddf4a64`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidReactor", abi = "InvalidReactor()")]
    pub struct InvalidReactor;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ResolvedOrderLibErrors {
        DeadlinePassed(DeadlinePassed),
        InvalidReactor(InvalidReactor),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ResolvedOrderLibErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <DeadlinePassed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeadlinePassed(decoded));
            }
            if let Ok(decoded)
                = <InvalidReactor as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidReactor(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ResolvedOrderLibErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeadlinePassed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ResolvedOrderLibErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DeadlinePassed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidReactor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ResolvedOrderLibErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeadlinePassed(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ResolvedOrderLibErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeadlinePassed> for ResolvedOrderLibErrors {
        fn from(value: DeadlinePassed) -> Self {
            Self::DeadlinePassed(value)
        }
    }
    impl ::core::convert::From<InvalidReactor> for ResolvedOrderLibErrors {
        fn from(value: InvalidReactor) -> Self {
            Self::InvalidReactor(value)
        }
    }
}
