pub use mock_expected_balance_lib::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod mock_expected_balance_lib {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("check"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("check"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("expected"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(::ethers::core::abi::ethabi::ParamType::Tuple(
                                ::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],
                            ),),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("struct ExpectedBalance[]"),
                        ),
                    },],
                    outputs: ::std::vec![],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("InsufficientOutput"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned("InsufficientOutput"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("actualBalance"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("expectedBalance"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                    ],
                },],
            )]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKEXPECTEDBALANCELIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x04\x84\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xE3\xA4\xD8k\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x02\xC5V[a\0EV[\0[a\0N\x81a\0QV[PV[`\0[\x81Q\x81\x10\x15a\x01\x1DW`\0\x82\x82\x81Q\x81\x10a\0qWa\0qa\x03\xA7V[` \x02` \x01\x01Q\x90P`\0a\0\xAE\x82`\0\x01Q\x83` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01!\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x81`@\x01Q\x81\x10\x15a\x01\x08W\x80\x82`@\x01Q`@Q\x7F,\x19\xB8\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xFD[PP\x80\x80a\x01\x15\x90a\x03\xD6V[\x91PPa\0TV[PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x01\\WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x161a\x01\xEFV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xEC\x91\x90a\x045V[\x90P[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02GWa\x02Ga\x01\xF5V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x94Wa\x02\x94a\x01\xF5V[`@R\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xC0W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x02\xD8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xF0W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x03\x04W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x03\x16Wa\x03\x16a\x01\xF5V[a\x03$\x84\x82`\x05\x1B\x01a\x02MV[\x81\x81R\x84\x81\x01\x92P``\x91\x82\x02\x84\x01\x85\x01\x91\x88\x83\x11\x15a\x03CW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x03\x9BW\x80\x85\x8A\x03\x12\x15a\x03`W`\0\x80\x81\xFD[a\x03ha\x02$V[a\x03q\x86a\x02\x9CV[\x81Ra\x03~\x87\x87\x01a\x02\x9CV[\x81\x88\x01R`@\x86\x81\x015\x90\x82\x01R\x84R\x93\x84\x01\x93\x92\x85\x01\x92a\x03HV[P\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x04.W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x04GW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x88=3\xBD\x1B\xB7\xF6\xA0\x96\x88\x03\xD4\xF5\x88Z\xCBpq\x95S+\x80~\xA1v\xA8QFX\x9D\xC9\x02dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKEXPECTEDBALANCELIB_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xE3\xA4\xD8k\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x02\xC5V[a\0EV[\0[a\0N\x81a\0QV[PV[`\0[\x81Q\x81\x10\x15a\x01\x1DW`\0\x82\x82\x81Q\x81\x10a\0qWa\0qa\x03\xA7V[` \x02` \x01\x01Q\x90P`\0a\0\xAE\x82`\0\x01Q\x83` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01!\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x81`@\x01Q\x81\x10\x15a\x01\x08W\x80\x82`@\x01Q`@Q\x7F,\x19\xB8\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xFD[PP\x80\x80a\x01\x15\x90a\x03\xD6V[\x91PPa\0TV[PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x01\\WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x161a\x01\xEFV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xEC\x91\x90a\x045V[\x90P[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02GWa\x02Ga\x01\xF5V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x94Wa\x02\x94a\x01\xF5V[`@R\x91\x90PV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\xC0W`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x02\xD8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xF0W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x03\x04W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x03\x16Wa\x03\x16a\x01\xF5V[a\x03$\x84\x82`\x05\x1B\x01a\x02MV[\x81\x81R\x84\x81\x01\x92P``\x91\x82\x02\x84\x01\x85\x01\x91\x88\x83\x11\x15a\x03CW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x03\x9BW\x80\x85\x8A\x03\x12\x15a\x03`W`\0\x80\x81\xFD[a\x03ha\x02$V[a\x03q\x86a\x02\x9CV[\x81Ra\x03~\x87\x87\x01a\x02\x9CV[\x81\x88\x01R`@\x86\x81\x015\x90\x82\x01R\x84R\x93\x84\x01\x93\x92\x85\x01\x92a\x03HV[P\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x04.W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x04GW`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x88=3\xBD\x1B\xB7\xF6\xA0\x96\x88\x03\xD4\xF5\x88Z\xCBpq\x95S+\x80~\xA1v\xA8QFX\x9D\xC9\x02dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKEXPECTEDBALANCELIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockExpectedBalanceLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockExpectedBalanceLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockExpectedBalanceLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockExpectedBalanceLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockExpectedBalanceLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockExpectedBalanceLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockExpectedBalanceLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKEXPECTEDBALANCELIB_ABI.clone(),
                client,
            ))
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
                MOCKEXPECTEDBALANCELIB_ABI.clone(),
                MOCKEXPECTEDBALANCELIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `check` (0xe3a4d86b) function
        pub fn check(
            &self,
            expected: ::std::vec::Vec<ExpectedBalance>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 164, 216, 107], expected)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockExpectedBalanceLib<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientOutput` with signature `InsufficientOutput(uint256,uint256)` and selector `0x2c19b8b8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "InsufficientOutput",
        abi = "InsufficientOutput(uint256,uint256)"
    )]
    pub struct InsufficientOutput {
        pub actual_balance: ::ethers::core::types::U256,
        pub expected_balance: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `check` function with signature `check((address,address,uint256)[])` and selector `0xe3a4d86b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "check", abi = "check((address,address,uint256)[])")]
    pub struct CheckCall {
        pub expected: ::std::vec::Vec<ExpectedBalance>,
    }
    ///`ExpectedBalance(address,address,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExpectedBalance {
        pub recipient: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub expected_balance: ::ethers::core::types::U256,
    }
}
