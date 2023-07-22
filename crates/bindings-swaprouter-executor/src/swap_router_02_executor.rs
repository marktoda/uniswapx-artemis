pub use swap_router_02_executor::*;
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
pub mod swap_router_02_executor {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_whitelistedCaller"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IReactor"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_swapRouter02"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISwapRouter02"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokensToApprove"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("multicallData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reactorCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reactorCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resolvedOrders"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("filler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fillData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unwrapWETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrapWETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CallerNotWhitelisted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CallerNotWhitelisted",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MsgSenderNotReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MsgSenderNotReactor",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeTransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NativeTransferFailed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SWAPROUTER02EXECUTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x14\xA18\x03\x80b\0\x14\xA1\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01+V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x82U`@Q\x84\x92\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\xA0R\x83\x81\x16`\xC0R\x81\x16`\x80\x81\x90R`@\x80Qc\x12\xA9)?`\xE2\x1B\x81R\x90QcJ\xA4\xA4\xFC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\0\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xFB\x91\x90b\0\x01\x93V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0RPb\0\x01\xBA\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01(W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01BW`\0\x80\xFD[\x84Qb\0\x01O\x81b\0\x01\x12V[` \x86\x01Q\x90\x94Pb\0\x01b\x81b\0\x01\x12V[`@\x86\x01Q\x90\x93Pb\0\x01u\x81b\0\x01\x12V[``\x86\x01Q\x90\x92Pb\0\x01\x88\x81b\0\x01\x12V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15b\0\x01\xA6W`\0\x80\xFD[\x81Qb\0\x01\xB3\x81b\0\x01\x12V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x12\x91b\0\x02\x10`\09`\0\x81\x81a\x03\x0B\x01Ra\x03\x97\x01R`\0a\x04\x14\x01R`\0a\x04T\x01R`\0\x81\x81a\x01\x86\x01R\x81\x81a\x02\t\x01R\x81\x81a\x04\xC8\x01Ra\x05F\x01Ra\x12\x91`\0\xF3\xFE`\x80`@R`\x046\x10a\0YW`\x005`\xE0\x1C\x80cc\xFB\x0B\x96\x14a\0eW\x80ci\r\x83 \x14a\0\x87W\x80c\x89\xA3\xF16\x14a\0\xA7W\x80c\x8D\xA5\xCB[\x14a\0\xC7W\x80c\x99C\xFA\x89\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01#W`\0\x80\xFD[6a\0`W\0[`\0\x80\xFD[4\x80\x15a\0qW`\0\x80\xFD[Pa\0\x85a\0\x806`\x04a\t?V[a\x01CV[\0[4\x80\x15a\0\x93W`\0\x80\xFD[Pa\0\x85a\0\xA26`\x04a\t\xBFV[a\x02\x92V[4\x80\x15a\0\xB3W`\0\x80\xFD[Pa\0\x85a\0\xC26`\x04a\t\xBFV[a\x02\xC9V[4\x80\x15a\0\xD3W`\0\x80\xFD[P`\0Ta\0\xE7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x0FW`\0\x80\xFD[Pa\0\x85a\x01\x1E6`\x04a\t\xE3V[a\x04\tV[4\x80\x15a\x01/W`\0\x80\xFD[Pa\0\x85a\x01>6`\x04a\t\xBFV[a\x06\x91V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x01\xF1Wa\x01\xDF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x19\x87\x87\x85\x81\x81\x10a\x01\xBAWa\x01\xBAa\n\xB7V[\x90P` \x02\x01` \x81\x01\x90a\x01\xCF\x91\x90a\t\xBFV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x07\x06V[\x80a\x01\xE9\x81a\n\xCDV[\x91PPa\x01yV[P`@Qc\x16\xB9\0w`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x02D\x90`\0\x19\x90\x86\x90\x86\x90`\x04\x01a\x0B\x1DV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x8B\x91\x90\x81\x01\x90a\x0C\xC2V[PPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[a\x02\xC6\x81Ga\x07\x8CV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03~\x91\x90a\r\xB3V[`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF7W=`\0\x80>=`\0\xFD[PPPPa\x04\x05\x82Ga\x07\x8CV[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04RW`@Qc\x93?\xE5/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\xA4W`@Qc\x8Cn]q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x04\xB3\x83\x85\x01\x85a\x0E\xA7V[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x05.Wa\x05\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x19\x85\x84\x81Q\x81\x10a\x04\xFCWa\x04\xFCa\n\xB7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x07\x06\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a\x05&\x81a\n\xCDV[\x91PPa\x04\xBAV[P`@Qc\x16\xB9\0w`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x05\x7F\x90`\0\x19\x90\x85\x90`\x04\x01a\x0FhV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\xC6\x91\x90\x81\x01\x90a\x0C\xC2V[P`\0[\x86\x81\x10\x15a\x06\x87W`\0\x88\x88\x83\x81\x81\x10a\x05\xE6Wa\x05\xE6a\n\xB7V[\x90P` \x02\x81\x01\x90a\x05\xF8\x91\x90a\x0F\xEAV[a\x06\x01\x90a\x11\xAEV[\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\x06rW`\0\x82`@\x01Q\x82\x81Q\x81\x10a\x06+Wa\x06+a\n\xB7V[` \x02` \x01\x01Q\x90Pa\x06_\x81`@\x01Q\x82` \x01Q\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x07\xE2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80a\x06j\x81a\n\xCDV[\x91PPa\x06\x06V[PP\x80\x80a\x06\x7F\x90a\n\xCDV[\x91PPa\x05\xCAV[PPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x01mV[PPPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x07\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x01a\x01mV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x08dW`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08BV[``\x91P[PP\x90P\x80a\x07\x86W`@Qc=,\xECo`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xDD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x01mV[`\0\x80\x83`\x1F\x84\x01\x12a\t\x06W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\x1DW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\t8W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\tUW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\tlW`\0\x80\xFD[a\tx\x88\x83\x89\x01a\x08\xF4V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\t\x91W`\0\x80\xFD[Pa\t\x9E\x87\x82\x88\x01a\x08\xF4V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xC6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\xD1W`\0\x80\xFD[\x815a\t\xDC\x81a\t\xAAV[\x93\x92PPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\t\xFBW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\n\x12W`\0\x80\xFD[a\n\x1E\x89\x83\x8A\x01a\x08\xF4V[\x90\x97P\x95P` \x88\x015\x91Pa\n3\x82a\t\xAAV[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\nIW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n]W`\0\x80\xFD[\x815\x81\x81\x11\x15a\nlW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\n~W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\n\xEDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0`@\x82\x01\x85\x83R` `@\x81\x85\x01R\x81\x85\x83R``\x85\x01\x90P``\x86`\x05\x1B\x86\x01\x01\x92P\x86`\0[\x87\x81\x10\x15a\x0B\xB6W\x86\x85\x03`_\x19\x01\x83R\x8156\x8A\x90\x03`\x1E\x19\x01\x81\x12a\x0BmW`\0\x80\xFD[\x89\x01\x84\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x88W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x0B\x97W`\0\x80\xFD[a\x0B\xA2\x87\x82\x84a\n\xF4V[\x96PPP\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a\x0BGV[P\x92\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xFCWa\x0B\xFCa\x0B\xC4V[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xFCWa\x0B\xFCa\x0B\xC4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CLWa\x0CLa\x0B\xC4V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0CmWa\x0Cma\x0B\xC4V[P`\x05\x1B` \x01\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\x90Wa\x0C\x90a\x0B\xC4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x0C\xB9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\xA1V[PP`\0\x91\x01RV[`\0` \x80\x83\x85\x03\x12\x15a\x0C\xD5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0C\xECW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\r\0W`\0\x80\xFD[\x81Qa\r\x13a\r\x0E\x82a\x0CTV[a\x0C$V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\r2W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\r\xA6W\x80Q\x85\x81\x11\x15a\rNW`\0\x80\x81\xFD[\x86\x01`?\x81\x01\x8B\x13a\r`W`\0\x80\x81\xFD[\x87\x81\x01Q`@a\rra\r\x0E\x83a\x0CwV[\x82\x81R\x8D\x82\x84\x86\x01\x01\x11\x15a\r\x87W`\0\x80\x81\xFD[a\r\x96\x83\x8C\x83\x01\x84\x87\x01a\x0C\x9EV[\x86RPPP\x91\x86\x01\x91\x86\x01a\r6V[P\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\r\xC5W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\r\xDDW`\0\x80\xFD[\x815a\r\xEBa\r\x0E\x82a\x0CwV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0E\0W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E.W`\0\x80\xFD[\x815` a\x0E>a\r\x0E\x83a\x0CTV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0E]W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0E\x9CW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x80W`\0\x80\x81\xFD[a\x0E\x8E\x89\x86\x83\x8B\x01\x01a\r\xCCV[\x84RP\x91\x83\x01\x91\x83\x01a\x0EaV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xBAW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\xD1W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\xE5W`\0\x80\xFD[\x815` a\x0E\xF5a\r\x0E\x83a\x0CTV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0F\x14W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0F;W\x855a\x0F,\x81a\t\xAAV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0F\x19V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x0FQW`\0\x80\xFD[Pa\x0F^\x85\x82\x86\x01a\x0E\x1DV[\x91PP\x92P\x92\x90PV[`\0`@\x82\x01\x84\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P``\x81`\x05\x1B\x87\x01\x01\x93P\x82\x87\x01`\0[\x82\x81\x10\x15a\x0F\xDCW\x87\x86\x03`_\x19\x01\x84R\x81Q\x80Q\x80\x88Ra\x0F\xBD\x81\x88\x8A\x01\x89\x85\x01a\x0C\x9EV[`\x1F\x01`\x1F\x19\x16\x96\x90\x96\x01\x85\x01\x95P\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x0F\x96V[P\x93\x98\x97PPPPPPPPV[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12a\x10\0W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x10\x1CW`\0\x80\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a\x10?Wa\x10?a\x0B\xC4V[\x81`@R\x82\x93P\x845\x91Pa\x10S\x82a\t\xAAV[\x90\x82R` \x84\x015\x90a\x10e\x82a\t\xAAV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x10\x8F\x82a\t\xAAV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x10\xA9W`\0\x80\xFD[Pa\x10\xB6\x85\x82\x86\x01a\r\xCCV[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x10\xD5W`\0\x80\xFD[a\x10\xDDa\x0B\xDAV[\x90P\x815a\x10\xEA\x81a\t\xAAV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x11\x19W`\0\x80\xFD[\x815` a\x11)a\r\x0E\x83a\x0CTV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x11HW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x11\xA1W\x81\x81\x8A\x03\x12\x15a\x11dW`\0\x80\x81\xFD[a\x11la\x0B\xDAV[\x815a\x11w\x81a\t\xAAV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x11\x90\x81a\t\xAAV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x11LV[P\x90\x97\x96PPPPPPPV[`\0`\xE0\x826\x03\x12\x15a\x11\xC0W`\0\x80\xFD[a\x11\xC8a\x0C\x02V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\xDFW`\0\x80\xFD[a\x11\xEB6\x83\x87\x01a\x10\nV[\x83Ra\x11\xFA6` \x87\x01a\x10\xC3V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x12\x13W`\0\x80\xFD[a\x12\x1F6\x83\x87\x01a\x11\x08V[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x128W`\0\x80\xFD[Pa\x12E6\x82\x86\x01a\r\xCCV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V\xFE\xA2dipfsX\"\x12 \xFDk9que)\xE1\xE4\xD73o\xF9?\x1E\xDA b\xE9\x952-_\rm\xC8\x03\x89w\xEB\x0C\xC8dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static SWAPROUTER02EXECUTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0YW`\x005`\xE0\x1C\x80cc\xFB\x0B\x96\x14a\0eW\x80ci\r\x83 \x14a\0\x87W\x80c\x89\xA3\xF16\x14a\0\xA7W\x80c\x8D\xA5\xCB[\x14a\0\xC7W\x80c\x99C\xFA\x89\x14a\x01\x03W\x80c\xF2\xFD\xE3\x8B\x14a\x01#W`\0\x80\xFD[6a\0`W\0[`\0\x80\xFD[4\x80\x15a\0qW`\0\x80\xFD[Pa\0\x85a\0\x806`\x04a\t?V[a\x01CV[\0[4\x80\x15a\0\x93W`\0\x80\xFD[Pa\0\x85a\0\xA26`\x04a\t\xBFV[a\x02\x92V[4\x80\x15a\0\xB3W`\0\x80\xFD[Pa\0\x85a\0\xC26`\x04a\t\xBFV[a\x02\xC9V[4\x80\x15a\0\xD3W`\0\x80\xFD[P`\0Ta\0\xE7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x0FW`\0\x80\xFD[Pa\0\x85a\x01\x1E6`\x04a\t\xE3V[a\x04\tV[4\x80\x15a\x01/W`\0\x80\xFD[Pa\0\x85a\x01>6`\x04a\t\xBFV[a\x06\x91V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x01\xF1Wa\x01\xDF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x19\x87\x87\x85\x81\x81\x10a\x01\xBAWa\x01\xBAa\n\xB7V[\x90P` \x02\x01` \x81\x01\x90a\x01\xCF\x91\x90a\t\xBFV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x07\x06V[\x80a\x01\xE9\x81a\n\xCDV[\x91PPa\x01yV[P`@Qc\x16\xB9\0w`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x02D\x90`\0\x19\x90\x86\x90\x86\x90`\x04\x01a\x0B\x1DV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x8B\x91\x90\x81\x01\x90a\x0C\xC2V[PPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[a\x02\xC6\x81Ga\x07\x8CV[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03~\x91\x90a\r\xB3V[`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF7W=`\0\x80>=`\0\xFD[PPPPa\x04\x05\x82Ga\x07\x8CV[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04RW`@Qc\x93?\xE5/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\xA4W`@Qc\x8Cn]q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x04\xB3\x83\x85\x01\x85a\x0E\xA7V[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x05.Wa\x05\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0\x19\x85\x84\x81Q\x81\x10a\x04\xFCWa\x04\xFCa\n\xB7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x07\x06\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a\x05&\x81a\n\xCDV[\x91PPa\x04\xBAV[P`@Qc\x16\xB9\0w`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x05\x7F\x90`\0\x19\x90\x85\x90`\x04\x01a\x0FhV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\xC6\x91\x90\x81\x01\x90a\x0C\xC2V[P`\0[\x86\x81\x10\x15a\x06\x87W`\0\x88\x88\x83\x81\x81\x10a\x05\xE6Wa\x05\xE6a\n\xB7V[\x90P` \x02\x81\x01\x90a\x05\xF8\x91\x90a\x0F\xEAV[a\x06\x01\x90a\x11\xAEV[\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\x06rW`\0\x82`@\x01Q\x82\x81Q\x81\x10a\x06+Wa\x06+a\n\xB7V[` \x02` \x01\x01Q\x90Pa\x06_\x81`@\x01Q\x82` \x01Q\x83`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x07\xE2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80a\x06j\x81a\n\xCDV[\x91PPa\x06\x06V[PP\x80\x80a\x06\x7F\x90a\n\xCDV[\x91PPa\x05\xCAV[PPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01m\x90a\n\x91V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x01mV[PPPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x07\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x01a\x01mV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x08dW`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08BV[``\x91P[PP\x90P\x80a\x07\x86W`@Qc=,\xECo`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xDD`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x01mV[`\0\x80\x83`\x1F\x84\x01\x12a\t\x06W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\x1DW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\t8W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\tUW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\tlW`\0\x80\xFD[a\tx\x88\x83\x89\x01a\x08\xF4V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\t\x91W`\0\x80\xFD[Pa\t\x9E\x87\x82\x88\x01a\x08\xF4V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xC6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\xD1W`\0\x80\xFD[\x815a\t\xDC\x81a\t\xAAV[\x93\x92PPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\t\xFBW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\n\x12W`\0\x80\xFD[a\n\x1E\x89\x83\x8A\x01a\x08\xF4V[\x90\x97P\x95P` \x88\x015\x91Pa\n3\x82a\t\xAAV[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\nIW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n]W`\0\x80\xFD[\x815\x81\x81\x11\x15a\nlW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\n~W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\n\xEDWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0`@\x82\x01\x85\x83R` `@\x81\x85\x01R\x81\x85\x83R``\x85\x01\x90P``\x86`\x05\x1B\x86\x01\x01\x92P\x86`\0[\x87\x81\x10\x15a\x0B\xB6W\x86\x85\x03`_\x19\x01\x83R\x8156\x8A\x90\x03`\x1E\x19\x01\x81\x12a\x0BmW`\0\x80\xFD[\x89\x01\x84\x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x88W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x0B\x97W`\0\x80\xFD[a\x0B\xA2\x87\x82\x84a\n\xF4V[\x96PPP\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a\x0BGV[P\x92\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xFCWa\x0B\xFCa\x0B\xC4V[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0B\xFCWa\x0B\xFCa\x0B\xC4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0CLWa\x0CLa\x0B\xC4V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0CmWa\x0Cma\x0B\xC4V[P`\x05\x1B` \x01\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\x90Wa\x0C\x90a\x0B\xC4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x0C\xB9W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\xA1V[PP`\0\x91\x01RV[`\0` \x80\x83\x85\x03\x12\x15a\x0C\xD5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0C\xECW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\r\0W`\0\x80\xFD[\x81Qa\r\x13a\r\x0E\x82a\x0CTV[a\x0C$V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\r2W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\r\xA6W\x80Q\x85\x81\x11\x15a\rNW`\0\x80\x81\xFD[\x86\x01`?\x81\x01\x8B\x13a\r`W`\0\x80\x81\xFD[\x87\x81\x01Q`@a\rra\r\x0E\x83a\x0CwV[\x82\x81R\x8D\x82\x84\x86\x01\x01\x11\x15a\r\x87W`\0\x80\x81\xFD[a\r\x96\x83\x8C\x83\x01\x84\x87\x01a\x0C\x9EV[\x86RPPP\x91\x86\x01\x91\x86\x01a\r6V[P\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\r\xC5W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\r\xDDW`\0\x80\xFD[\x815a\r\xEBa\r\x0E\x82a\x0CwV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0E\0W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E.W`\0\x80\xFD[\x815` a\x0E>a\r\x0E\x83a\x0CTV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0E]W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0E\x9CW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x80W`\0\x80\x81\xFD[a\x0E\x8E\x89\x86\x83\x8B\x01\x01a\r\xCCV[\x84RP\x91\x83\x01\x91\x83\x01a\x0EaV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xBAW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\xD1W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\xE5W`\0\x80\xFD[\x815` a\x0E\xF5a\r\x0E\x83a\x0CTV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0F\x14W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0F;W\x855a\x0F,\x81a\t\xAAV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0F\x19V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x0FQW`\0\x80\xFD[Pa\x0F^\x85\x82\x86\x01a\x0E\x1DV[\x91PP\x92P\x92\x90PV[`\0`@\x82\x01\x84\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P``\x81`\x05\x1B\x87\x01\x01\x93P\x82\x87\x01`\0[\x82\x81\x10\x15a\x0F\xDCW\x87\x86\x03`_\x19\x01\x84R\x81Q\x80Q\x80\x88Ra\x0F\xBD\x81\x88\x8A\x01\x89\x85\x01a\x0C\x9EV[`\x1F\x01`\x1F\x19\x16\x96\x90\x96\x01\x85\x01\x95P\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x0F\x96V[P\x93\x98\x97PPPPPPPPV[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12a\x10\0W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x10\x1CW`\0\x80\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a\x10?Wa\x10?a\x0B\xC4V[\x81`@R\x82\x93P\x845\x91Pa\x10S\x82a\t\xAAV[\x90\x82R` \x84\x015\x90a\x10e\x82a\t\xAAV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x10\x8F\x82a\t\xAAV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x10\xA9W`\0\x80\xFD[Pa\x10\xB6\x85\x82\x86\x01a\r\xCCV[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x10\xD5W`\0\x80\xFD[a\x10\xDDa\x0B\xDAV[\x90P\x815a\x10\xEA\x81a\t\xAAV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x11\x19W`\0\x80\xFD[\x815` a\x11)a\r\x0E\x83a\x0CTV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x11HW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x11\xA1W\x81\x81\x8A\x03\x12\x15a\x11dW`\0\x80\x81\xFD[a\x11la\x0B\xDAV[\x815a\x11w\x81a\t\xAAV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x11\x90\x81a\t\xAAV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x11LV[P\x90\x97\x96PPPPPPPV[`\0`\xE0\x826\x03\x12\x15a\x11\xC0W`\0\x80\xFD[a\x11\xC8a\x0C\x02V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\xDFW`\0\x80\xFD[a\x11\xEB6\x83\x87\x01a\x10\nV[\x83Ra\x11\xFA6` \x87\x01a\x10\xC3V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x12\x13W`\0\x80\xFD[a\x12\x1F6\x83\x87\x01a\x11\x08V[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x128W`\0\x80\xFD[Pa\x12E6\x82\x86\x01a\r\xCCV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V\xFE\xA2dipfsX\"\x12 \xFDk9que)\xE1\xE4\xD73o\xF9?\x1E\xDA b\xE9\x952-_\rm\xC8\x03\x89w\xEB\x0C\xC8dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static SWAPROUTER02EXECUTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SwapRouter02Executor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SwapRouter02Executor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SwapRouter02Executor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SwapRouter02Executor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SwapRouter02Executor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SwapRouter02Executor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SwapRouter02Executor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SWAPROUTER02EXECUTOR_ABI.clone(),
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
                SWAPROUTER02EXECUTOR_ABI.clone(),
                SWAPROUTER02EXECUTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `multicall` (0x63fb0b96) function
        pub fn multicall(
            &self,
            tokens_to_approve: ::std::vec::Vec<::ethers::core::types::Address>,
            multicall_data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 251, 11, 150], (tokens_to_approve, multicall_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactorCallback` (0x9943fa89) function
        pub fn reactor_callback(
            &self,
            resolved_orders: ::std::vec::Vec<ResolvedOrder>,
            filler: ::ethers::core::types::Address,
            fill_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 67, 250, 137], (resolved_orders, filler, fill_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH` (0x89a3f136) function
        pub fn unwrap_weth(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 163, 241, 54], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawETH` (0x690d8320) function
        pub fn withdraw_eth(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 13, 131, 32], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SwapRouter02Executor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CallerNotWhitelisted` with signature `CallerNotWhitelisted()` and selector `0x8c6e5d71`
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
    #[etherror(name = "CallerNotWhitelisted", abi = "CallerNotWhitelisted()")]
    pub struct CallerNotWhitelisted;
    ///Custom Error type `MsgSenderNotReactor` with signature `MsgSenderNotReactor()` and selector `0x933fe52f`
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
    #[etherror(name = "MsgSenderNotReactor", abi = "MsgSenderNotReactor()")]
    pub struct MsgSenderNotReactor;
    ///Custom Error type `NativeTransferFailed` with signature `NativeTransferFailed()` and selector `0xf4b3b1bc`
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
    #[etherror(name = "NativeTransferFailed", abi = "NativeTransferFailed()")]
    pub struct NativeTransferFailed;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SwapRouter02ExecutorErrors {
        CallerNotWhitelisted(CallerNotWhitelisted),
        MsgSenderNotReactor(MsgSenderNotReactor),
        NativeTransferFailed(NativeTransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SwapRouter02ExecutorErrors {
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
                = <CallerNotWhitelisted as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CallerNotWhitelisted(decoded));
            }
            if let Ok(decoded)
                = <MsgSenderNotReactor as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MsgSenderNotReactor(decoded));
            }
            if let Ok(decoded)
                = <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NativeTransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SwapRouter02ExecutorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CallerNotWhitelisted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MsgSenderNotReactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SwapRouter02ExecutorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CallerNotWhitelisted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MsgSenderNotReactor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NativeTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SwapRouter02ExecutorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerNotWhitelisted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MsgSenderNotReactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NativeTransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SwapRouter02ExecutorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CallerNotWhitelisted> for SwapRouter02ExecutorErrors {
        fn from(value: CallerNotWhitelisted) -> Self {
            Self::CallerNotWhitelisted(value)
        }
    }
    impl ::core::convert::From<MsgSenderNotReactor> for SwapRouter02ExecutorErrors {
        fn from(value: MsgSenderNotReactor) -> Self {
            Self::MsgSenderNotReactor(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for SwapRouter02ExecutorErrors {
        fn from(value: NativeTransferFailed) -> Self {
            Self::NativeTransferFailed(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(address[],bytes[])` and selector `0x63fb0b96`
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
    #[ethcall(name = "multicall", abi = "multicall(address[],bytes[])")]
    pub struct MulticallCall {
        pub tokens_to_approve: ::std::vec::Vec<::ethers::core::types::Address>,
        pub multicall_data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `reactorCallback` function with signature `reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],address,bytes)` and selector `0x9943fa89`
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
    #[ethcall(
        name = "reactorCallback",
        abi = "reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],address,bytes)"
    )]
    pub struct ReactorCallbackCall {
        pub resolved_orders: ::std::vec::Vec<ResolvedOrder>,
        pub filler: ::ethers::core::types::Address,
        pub fill_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapWETH` function with signature `unwrapWETH(address)` and selector `0x89a3f136`
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
    #[ethcall(name = "unwrapWETH", abi = "unwrapWETH(address)")]
    pub struct UnwrapWETHCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawETH` function with signature `withdrawETH(address)` and selector `0x690d8320`
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
    #[ethcall(name = "withdrawETH", abi = "withdrawETH(address)")]
    pub struct WithdrawETHCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SwapRouter02ExecutorCalls {
        Multicall(MulticallCall),
        Owner(OwnerCall),
        ReactorCallback(ReactorCallbackCall),
        TransferOwnership(TransferOwnershipCall),
        UnwrapWETH(UnwrapWETHCall),
        WithdrawETH(WithdrawETHCall),
    }
    impl ::ethers::core::abi::AbiDecode for SwapRouter02ExecutorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReactorCallback(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UnwrapWETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnwrapWETH(decoded));
            }
            if let Ok(decoded)
                = <WithdrawETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SwapRouter02ExecutorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReactorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SwapRouter02ExecutorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawETH(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MulticallCall> for SwapRouter02ExecutorCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SwapRouter02ExecutorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall> for SwapRouter02ExecutorCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for SwapRouter02ExecutorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnwrapWETHCall> for SwapRouter02ExecutorCalls {
        fn from(value: UnwrapWETHCall) -> Self {
            Self::UnwrapWETH(value)
        }
    }
    impl ::core::convert::From<WithdrawETHCall> for SwapRouter02ExecutorCalls {
        fn from(value: WithdrawETHCall) -> Self {
            Self::WithdrawETH(value)
        }
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
