pub use mock_swapper::*;
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
pub mod mock_swapper {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static MOCKSWAPPER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        1,
        168,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        225,
        242,
        28,
        103,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        1,
        13,
        86,
        91,
        97,
        0,
        69,
        86,
        91,
        0,
        91,
        96,
        64,
        81,
        127,
        9,
        94,
        167,
        179,
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        82,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        131,
        129,
        22,
        96,
        4,
        131,
        1,
        82,
        96,
        36,
        130,
        1,
        131,
        144,
        82,
        132,
        22,
        144,
        99,
        9,
        94,
        167,
        179,
        144,
        96,
        68,
        1,
        96,
        32,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        90,
        241,
        21,
        128,
        21,
        97,
        0,
        186,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        80,
        80,
        96,
        64,
        81,
        61,
        96,
        31,
        25,
        96,
        31,
        130,
        1,
        22,
        130,
        1,
        128,
        96,
        64,
        82,
        80,
        129,
        1,
        144,
        97,
        0,
        222,
        145,
        144,
        97,
        1,
        73,
        86,
        91,
        80,
        80,
        80,
        80,
        86,
        91,
        128,
        53,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        22,
        129,
        20,
        97,
        1,
        8,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        96,
        96,
        132,
        134,
        3,
        18,
        21,
        97,
        1,
        34,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        1,
        43,
        132,
        97,
        0,
        228,
        86,
        91,
        146,
        80,
        97,
        1,
        57,
        96,
        32,
        133,
        1,
        97,
        0,
        228,
        86,
        91,
        145,
        80,
        96,
        64,
        132,
        1,
        53,
        144,
        80,
        146,
        80,
        146,
        80,
        146,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        91,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        81,
        128,
        21,
        21,
        129,
        20,
        97,
        1,
        107,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
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
        18,
        135,
        178,
        199,
        25,
        12,
        155,
        122,
        137,
        194,
        200,
        234,
        88,
        183,
        96,
        105,
        36,
        101,
        115,
        185,
        90,
        226,
        207,
        148,
        224,
        24,
        6,
        44,
        194,
        70,
        50,
        95,
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
    pub static MOCKSWAPPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        225,
        242,
        28,
        103,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        1,
        13,
        86,
        91,
        97,
        0,
        69,
        86,
        91,
        0,
        91,
        96,
        64,
        81,
        127,
        9,
        94,
        167,
        179,
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
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        82,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        131,
        129,
        22,
        96,
        4,
        131,
        1,
        82,
        96,
        36,
        130,
        1,
        131,
        144,
        82,
        132,
        22,
        144,
        99,
        9,
        94,
        167,
        179,
        144,
        96,
        68,
        1,
        96,
        32,
        96,
        64,
        81,
        128,
        131,
        3,
        129,
        96,
        0,
        135,
        90,
        241,
        21,
        128,
        21,
        97,
        0,
        186,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        80,
        80,
        96,
        64,
        81,
        61,
        96,
        31,
        25,
        96,
        31,
        130,
        1,
        22,
        130,
        1,
        128,
        96,
        64,
        82,
        80,
        129,
        1,
        144,
        97,
        0,
        222,
        145,
        144,
        97,
        1,
        73,
        86,
        91,
        80,
        80,
        80,
        80,
        86,
        91,
        128,
        53,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        129,
        22,
        129,
        20,
        97,
        1,
        8,
        87,
        96,
        0,
        128,
        253,
        91,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        0,
        96,
        96,
        132,
        134,
        3,
        18,
        21,
        97,
        1,
        34,
        87,
        96,
        0,
        128,
        253,
        91,
        97,
        1,
        43,
        132,
        97,
        0,
        228,
        86,
        91,
        146,
        80,
        97,
        1,
        57,
        96,
        32,
        133,
        1,
        97,
        0,
        228,
        86,
        91,
        145,
        80,
        96,
        64,
        132,
        1,
        53,
        144,
        80,
        146,
        80,
        146,
        80,
        146,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        91,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        81,
        128,
        21,
        21,
        129,
        20,
        97,
        1,
        107,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
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
        18,
        135,
        178,
        199,
        25,
        12,
        155,
        122,
        137,
        194,
        200,
        234,
        88,
        183,
        96,
        105,
        36,
        101,
        115,
        185,
        90,
        226,
        207,
        148,
        224,
        24,
        6,
        44,
        194,
        70,
        50,
        95,
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
    pub static MOCKSWAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockSwapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockSwapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockSwapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockSwapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockSwapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(MockSwapper)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockSwapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKSWAPPER_ABI.clone(),
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
                MOCKSWAPPER_ABI.clone(),
                MOCKSWAPPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `approve` (0xe1f21c67) function
        pub fn approve(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 242, 28, 103], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockSwapper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,address,uint256)` and selector `0xe1f21c67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,address,uint256)")]
    pub struct ApproveCall {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
}
