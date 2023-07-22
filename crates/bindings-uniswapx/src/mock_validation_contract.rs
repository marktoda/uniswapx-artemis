pub use mock_validation_contract::*;
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
pub mod mock_validation_contract {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("setValid"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setValid"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_valid"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("valid"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("valid"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        256usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("MockValidationError"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned("MockValidationError",),
                    inputs: ::std::vec![],
                },],
            )]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKVALIDATIONCONTRACT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x05|\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cld\xED\xEE\x14a\0FW\x80cn\x84\xBA+\x14a\0\x87W\x80c\xC1\x99\x12\x19\x14a\0\x9AW[`\0\x80\xFD[a\0\x85a\0T6`\x04a\0\xFBV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[a\0\x85a\0\x956`\x04a\x04`V[a\0\xBBV[`\0Ta\0\xA7\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0T`\xFF\x16a\0\xF7W`@Q\x7F\xB3\xCA.(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\0` \x82\x84\x03\x12\x15a\x01\rW`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01FW`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9BWa\x01\x9Ba\x01IV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9BWa\x01\x9Ba\x01IV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x0BWa\x02\x0Ba\x01IV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x02$W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02>Wa\x02>a\x01IV[a\x02o` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x01\xC4V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x02\x84W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x02\xB3W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x02\xD7Wa\x02\xD7a\x01IV[\x81`@R\x82\x93P\x845\x91Pa\x02\xEB\x82a\x01$V[\x90\x82R` \x84\x015\x90a\x02\xFD\x82a\x01$V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x03'\x82a\x01$V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x03AW`\0\x80\xFD[Pa\x03N\x85\x82\x86\x01a\x02\x13V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x03mW`\0\x80\xFD[a\x03ua\x01xV[\x90P\x815a\x03\x82\x81a\x01$V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x03\xB1W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xCDWa\x03\xCDa\x01IV[a\x03\xDB\x81\x83`\x05\x1B\x01a\x01\xC4V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x03\xFAW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x04SW\x81\x81\x8A\x03\x12\x15a\x04\x16W`\0\x80\x81\xFD[a\x04\x1Ea\x01xV[\x815a\x04)\x81a\x01$V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x04B\x81a\x01$V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x03\xFEV[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04sW`\0\x80\xFD[\x825a\x04~\x81a\x01$V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x9BW`\0\x80\xFD[\x90\x84\x01\x90`\xE0\x82\x87\x03\x12\x15a\x04\xAFW`\0\x80\xFD[a\x04\xB7a\x01\xA1V[\x825\x82\x81\x11\x15a\x04\xC6W`\0\x80\xFD[a\x04\xD2\x88\x82\x86\x01a\x02\xA1V[\x82RPa\x04\xE2\x87` \x85\x01a\x03[V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x04\xF9W`\0\x80\xFD[a\x05\x05\x88\x82\x86\x01a\x03\xA0V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x05\x1DW`\0\x80\xFD[a\x05)\x88\x82\x86\x01a\x02\x13V[``\x83\x01RP`\xC0\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \x9C\xBE5n\xC53L\x98\xDA&\x16\x08\xE5AP\xB7z\xC5z\x97\"\xBC\xAB\x1F\0\xDFB\x7F\0\xA3h\xC9dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKVALIDATIONCONTRACT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cld\xED\xEE\x14a\0FW\x80cn\x84\xBA+\x14a\0\x87W\x80c\xC1\x99\x12\x19\x14a\0\x9AW[`\0\x80\xFD[a\0\x85a\0T6`\x04a\0\xFBV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[a\0\x85a\0\x956`\x04a\x04`V[a\0\xBBV[`\0Ta\0\xA7\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0T`\xFF\x16a\0\xF7W`@Q\x7F\xB3\xCA.(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\0` \x82\x84\x03\x12\x15a\x01\rW`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01FW`\0\x80\xFD[PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9BWa\x01\x9Ba\x01IV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x9BWa\x01\x9Ba\x01IV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x0BWa\x02\x0Ba\x01IV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x02$W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02>Wa\x02>a\x01IV[a\x02o` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x01\xC4V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x02\x84W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x02\xB3W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x02\xD7Wa\x02\xD7a\x01IV[\x81`@R\x82\x93P\x845\x91Pa\x02\xEB\x82a\x01$V[\x90\x82R` \x84\x015\x90a\x02\xFD\x82a\x01$V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x03'\x82a\x01$V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x03AW`\0\x80\xFD[Pa\x03N\x85\x82\x86\x01a\x02\x13V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x03mW`\0\x80\xFD[a\x03ua\x01xV[\x90P\x815a\x03\x82\x81a\x01$V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x03\xB1W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xCDWa\x03\xCDa\x01IV[a\x03\xDB\x81\x83`\x05\x1B\x01a\x01\xC4V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x03\xFAW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x04SW\x81\x81\x8A\x03\x12\x15a\x04\x16W`\0\x80\x81\xFD[a\x04\x1Ea\x01xV[\x815a\x04)\x81a\x01$V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x04B\x81a\x01$V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x03\xFEV[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04sW`\0\x80\xFD[\x825a\x04~\x81a\x01$V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x9BW`\0\x80\xFD[\x90\x84\x01\x90`\xE0\x82\x87\x03\x12\x15a\x04\xAFW`\0\x80\xFD[a\x04\xB7a\x01\xA1V[\x825\x82\x81\x11\x15a\x04\xC6W`\0\x80\xFD[a\x04\xD2\x88\x82\x86\x01a\x02\xA1V[\x82RPa\x04\xE2\x87` \x85\x01a\x03[V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x04\xF9W`\0\x80\xFD[a\x05\x05\x88\x82\x86\x01a\x03\xA0V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x05\x1DW`\0\x80\xFD[a\x05)\x88\x82\x86\x01a\x02\x13V[``\x83\x01RP`\xC0\x83\x015`\x80\x82\x01R\x80\x93PPPP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \x9C\xBE5n\xC53L\x98\xDA&\x16\x08\xE5AP\xB7z\xC5z\x97\"\xBC\xAB\x1F\0\xDFB\x7F\0\xA3h\xC9dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKVALIDATIONCONTRACT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockValidationContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockValidationContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockValidationContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockValidationContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockValidationContract<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockValidationContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockValidationContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKVALIDATIONCONTRACT_ABI.clone(),
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
                MOCKVALIDATIONCONTRACT_ABI.clone(),
                MOCKVALIDATIONCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `setValid` (0x6c64edee) function
        pub fn set_valid(&self, valid: bool) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 100, 237, 238], valid)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `valid` (0xc1991219) function
        pub fn valid(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([193, 153, 18, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validate` (0x6e84ba2b) function
        pub fn validate(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ResolvedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 132, 186, 43], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockValidationContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `MockValidationError` with signature `MockValidationError()` and selector `0xb3ca2e28`
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
    #[etherror(name = "MockValidationError", abi = "MockValidationError()")]
    pub struct MockValidationError;
    ///Container type for all input parameters for the `setValid` function with signature `setValid(bool)` and selector `0x6c64edee`
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
    #[ethcall(name = "setValid", abi = "setValid(bool)")]
    pub struct SetValidCall {
        pub valid: bool,
    }
    ///Container type for all input parameters for the `valid` function with signature `valid()` and selector `0xc1991219`
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
    #[ethcall(name = "valid", abi = "valid()")]
    pub struct ValidCall;
    ///Container type for all input parameters for the `validate` function with signature `validate(address,((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))` and selector `0x6e84ba2b`
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
    #[ethcall(
        name = "validate",
        abi = "validate(address,((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))"
    )]
    pub struct ValidateCall(pub ::ethers::core::types::Address, pub ResolvedOrder);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockValidationContractCalls {
        SetValid(SetValidCall),
        Valid(ValidCall),
        Validate(ValidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockValidationContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SetValidCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetValid(decoded));
            }
            if let Ok(decoded) = <ValidCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Valid(decoded));
            }
            if let Ok(decoded) = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Validate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockValidationContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SetValid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Valid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Validate(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockValidationContractCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetValid(element) => ::core::fmt::Display::fmt(element, f),
                Self::Valid(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SetValidCall> for MockValidationContractCalls {
        fn from(value: SetValidCall) -> Self {
            Self::SetValid(value)
        }
    }
    impl ::core::convert::From<ValidCall> for MockValidationContractCalls {
        fn from(value: ValidCall) -> Self {
            Self::Valid(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for MockValidationContractCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    ///Container type for all return fields from the `valid` function with signature `valid()` and selector `0xc1991219`
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
    pub struct ValidReturn(pub bool);
}
