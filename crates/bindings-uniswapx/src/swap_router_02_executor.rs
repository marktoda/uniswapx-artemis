pub use swap_router_02_executor::*;
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
    pub static SWAPROUTER02EXECUTOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1Ae8\x03\x80b\0\x1Ae\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01+V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x82U`@Q\x84\x92\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\xA0R\x83\x81\x16`\xC0R\x81\x16`\x80\x81\x90R`@\x80Qc\x12\xA9)?`\xE2\x1B\x81R\x90QcJ\xA4\xA4\xFC\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\0\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xFB\x91\x90b\0\x01\x93V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0RPb\0\x01\xBA\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01(W`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01BW`\0\x80\xFD[\x84Qb\0\x01O\x81b\0\x01\x12V[` \x86\x01Q\x90\x94Pb\0\x01b\x81b\0\x01\x12V[`@\x86\x01Q\x90\x93Pb\0\x01u\x81b\0\x01\x12V[``\x86\x01Q\x90\x92Pb\0\x01\x88\x81b\0\x01\x12V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15b\0\x01\xA6W`\0\x80\xFD[\x81Qb\0\x01\xB3\x81b\0\x01\x12V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x18Ub\0\x02\x10`\09`\0\x81\x81a\x04\xDC\x01Ra\x05\x8E\x01R`\0a\x06%\x01R`\0a\x06~\x01R`\0\x81\x81a\x02\x03\x01R\x81\x81a\x02\xD7\x01R\x81\x81a\x07%\x01Ra\x07\xF4\x01Ra\x18U`\0\xF3\xFE`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0CW\x80c\x8D\xA5\xCB[\x14a\0\xD7W\x80c\x99C\xFA\x89\x14a\x01-W\x80c\xF2\xFD\xE3\x8B\x14a\x01MW`\0\x80\xFD[\x80cc\xFB\x0B\x96\x14a\0uW\x80ci\r\x83 \x14a\0\x97W\x80c\x89\xA3\xF16\x14a\0\xB7W`\0\x80\xFD[6a\0pW\0[`\0\x80\xFD[4\x80\x15a\0\x81W`\0\x80\xFD[Pa\0\x95a\0\x906`\x04a\r\xB4V[a\x01mV[\0[4\x80\x15a\0\xA3W`\0\x80\xFD[Pa\0\x95a\0\xB26`\x04a\x0EBV[a\x03\x9CV[4\x80\x15a\0\xC3W`\0\x80\xFD[Pa\0\x95a\0\xD26`\x04a\x0EBV[a\x04*V[4\x80\x15a\0\xE3W`\0\x80\xFD[P`\0Ta\x01\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x019W`\0\x80\xFD[Pa\0\x95a\x01H6`\x04a\x0EfV[a\x06\rV[4\x80\x15a\x01YW`\0\x80\xFD[Pa\0\x95a\x01h6`\x04a\x0EBV[a\t\x88V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x01\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x02\x99Wa\x02\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x87\x85\x81\x81\x10a\x02UWa\x02Ua\x0F\x15V[\x90P` \x02\x01` \x81\x01\x90a\x02j\x91\x90a\x0EBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90a\nyV[\x80a\x02\x91\x81a\x0FDV[\x91PPa\x01\xF6V[P`@Q\x7FZ\xE4\x01\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x030\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x86\x90\x86\x90`\x04\x01a\x0F\xECV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03\x95\x91\x90\x81\x01\x90a\x12'V[PPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[a\x04'\x81Ga\x0BNV[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\xABW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x058W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\\\x91\x90a\x13\x19V[`@Q\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xFBW=`\0\x80>=`\0\xFD[PPPPa\x06\t\x82Ga\x0BNV[PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06|W`@Q\x7F\x93?\xE5/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\x01W`@Q\x7F\x8Cn]q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x07\x10\x83\x85\x01\x85a\x14\x0EV[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x07\xB6Wa\x07\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x84\x81Q\x81\x10a\x07wWa\x07wa\x0F\x15V[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\ny\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a\x07\xAE\x81a\x0FDV[\x91PPa\x07\x17V[P`@Q\x7FZ\xE4\x01\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x08K\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x85\x90`\x04\x01a\x14\xD0V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\xB0\x91\x90\x81\x01\x90a\x12'V[P`\0[\x86\x81\x10\x15a\t~W`\0\x88\x88\x83\x81\x81\x10a\x08\xD0Wa\x08\xD0a\x0F\x15V[\x90P` \x02\x81\x01\x90a\x08\xE2\x91\x90a\x15\x8EV[a\x08\xEB\x90a\x17qV[\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\tiW`\0\x82`@\x01Q\x82\x81Q\x81\x10a\t\x15Wa\t\x15a\x0F\x15V[` \x02` \x01\x01Q\x90Pa\tV\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xC8\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80a\ta\x81a\x0FDV[\x91PPa\x08\xF0V[PP\x80\x80a\tv\x90a\x0FDV[\x91PPa\x08\xB4V[PPPPPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\n\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FAPPROVE_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[PPPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x0B\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FETH_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x0C}W`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0C=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CBV[``\x91P[PP\x90P\x80a\x0BHW`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xC3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[`\0\x80\x83`\x1F\x84\x01\x12a\rzW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x92W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\r\xADW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\r\xCAW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xE2W`\0\x80\xFD[a\r\xEE\x88\x83\x89\x01a\rhV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x0E\x07W`\0\x80\xFD[Pa\x0E\x14\x87\x82\x88\x01a\rhV[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04'W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0ETW`\0\x80\xFD[\x815a\x0E_\x81a\x0E V[\x93\x92PPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x0E~W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x96W`\0\x80\xFD[a\x0E\xA2\x89\x83\x8A\x01a\rhV[\x90\x97P\x95P` \x88\x015\x91Pa\x0E\xB7\x82a\x0E V[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\x0E\xCDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x0E\xE1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0E\xF0W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x0F\x02W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0F\x9CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0`@\x82\x01\x85\x83R` `@\x81\x85\x01R\x81\x85\x83R``\x85\x01\x90P``\x86`\x05\x1B\x86\x01\x01\x92P\x86`\0[\x87\x81\x10\x15a\x10\xC1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x83R\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x8A6\x03\x01\x81\x12a\x10wW`\0\x80\xFD[\x89\x01\x84\x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x93W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x10\xA2W`\0\x80\xFD[a\x10\xAD\x87\x82\x84a\x0F\xA3V[\x96PPP\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a\x10\x16V[P\x92\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11!Wa\x11!a\x10\xCFV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11!Wa\x11!a\x10\xCFV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\x91Wa\x11\x91a\x10\xCFV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xB3Wa\x11\xB3a\x10\xCFV[P`\x05\x1B` \x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xD7Wa\x11\xD7a\x10\xCFV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x12\x1EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\x06V[PP`\0\x91\x01RV[`\0` \x80\x83\x85\x03\x12\x15a\x12:W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12RW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12fW`\0\x80\xFD[\x81Qa\x12ya\x12t\x82a\x11\x99V[a\x11JV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x12\x98W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x13\x0CW\x80Q\x85\x81\x11\x15a\x12\xB4W`\0\x80\x81\xFD[\x86\x01`?\x81\x01\x8B\x13a\x12\xC6W`\0\x80\x81\xFD[\x87\x81\x01Q`@a\x12\xD8a\x12t\x83a\x11\xBDV[\x82\x81R\x8D\x82\x84\x86\x01\x01\x11\x15a\x12\xEDW`\0\x80\x81\xFD[a\x12\xFC\x83\x8C\x83\x01\x84\x87\x01a\x12\x03V[\x86RPPP\x91\x86\x01\x91\x86\x01a\x12\x9CV[P\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x13+W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x13CW`\0\x80\xFD[\x815a\x13Qa\x12t\x82a\x11\xBDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x13fW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x13\x94W`\0\x80\xFD[\x815` a\x13\xA4a\x12t\x83a\x11\x99V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x13\xC3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x14\x03W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xE7W`\0\x80\x81\xFD[a\x13\xF5\x89\x86\x83\x8B\x01\x01a\x132V[\x84RP\x91\x83\x01\x91\x83\x01a\x13\xC7V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14!W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x149W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x14MW`\0\x80\xFD[\x815` a\x14]a\x12t\x83a\x11\x99V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x14|W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x14\xA3W\x855a\x14\x94\x81a\x0E V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x14\x81V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x14\xB9W`\0\x80\xFD[Pa\x14\xC6\x85\x82\x86\x01a\x13\x83V[\x91PP\x92P\x92\x90PV[`\0`@\x82\x01\x84\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P``\x81`\x05\x1B\x87\x01\x01\x93P\x82\x87\x01`\0[\x82\x81\x10\x15a\x15\x80W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x84R\x81Q\x80Q\x80\x88Ra\x15C\x81\x88\x8A\x01\x89\x85\x01a\x12\x03V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x96\x90\x96\x01\x85\x01\x95P\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x14\xFEV[P\x93\x98\x97PPPPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x836\x03\x01\x81\x12a\x15\xC2W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x15\xDEW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x16\x02Wa\x16\x02a\x10\xCFV[\x81`@R\x82\x93P\x845\x91Pa\x16\x16\x82a\x0E V[\x90\x82R` \x84\x015\x90a\x16(\x82a\x0E V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x16R\x82a\x0E V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x16lW`\0\x80\xFD[Pa\x16y\x85\x82\x86\x01a\x132V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x16\x98W`\0\x80\xFD[a\x16\xA0a\x10\xFEV[\x90P\x815a\x16\xAD\x81a\x0E V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x16\xDCW`\0\x80\xFD[\x815` a\x16\xECa\x12t\x83a\x11\x99V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x17\x0BW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x17dW\x81\x81\x8A\x03\x12\x15a\x17'W`\0\x80\x81\xFD[a\x17/a\x10\xFEV[\x815a\x17:\x81a\x0E V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x17S\x81a\x0E V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x17\x0FV[P\x90\x97\x96PPPPPPPV[`\0`\xE0\x826\x03\x12\x15a\x17\x83W`\0\x80\xFD[a\x17\x8Ba\x11'V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xA3W`\0\x80\xFD[a\x17\xAF6\x83\x87\x01a\x15\xCCV[\x83Ra\x17\xBE6` \x87\x01a\x16\x86V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x17\xD7W`\0\x80\xFD[a\x17\xE36\x83\x87\x01a\x16\xCBV[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x17\xFCW`\0\x80\xFD[Pa\x18\t6\x82\x86\x01a\x132V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V\xFE\xA2dipfsX\"\x12 \x86Fa\x13\xE8\x06\x0Bz\x84\x07\x13qz+\x14\xC7\xF0D\xD0\xE4\x90x\xAFl\xE43+F\x04\xC5hzdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SWAPROUTER02EXECUTOR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0iW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0CW\x80c\x8D\xA5\xCB[\x14a\0\xD7W\x80c\x99C\xFA\x89\x14a\x01-W\x80c\xF2\xFD\xE3\x8B\x14a\x01MW`\0\x80\xFD[\x80cc\xFB\x0B\x96\x14a\0uW\x80ci\r\x83 \x14a\0\x97W\x80c\x89\xA3\xF16\x14a\0\xB7W`\0\x80\xFD[6a\0pW\0[`\0\x80\xFD[4\x80\x15a\0\x81W`\0\x80\xFD[Pa\0\x95a\0\x906`\x04a\r\xB4V[a\x01mV[\0[4\x80\x15a\0\xA3W`\0\x80\xFD[Pa\0\x95a\0\xB26`\x04a\x0EBV[a\x03\x9CV[4\x80\x15a\0\xC3W`\0\x80\xFD[Pa\0\x95a\0\xD26`\x04a\x0EBV[a\x04*V[4\x80\x15a\0\xE3W`\0\x80\xFD[P`\0Ta\x01\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x019W`\0\x80\xFD[Pa\0\x95a\x01H6`\x04a\x0EfV[a\x06\rV[4\x80\x15a\x01YW`\0\x80\xFD[Pa\0\x95a\x01h6`\x04a\x0EBV[a\t\x88V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x01\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x02\x99Wa\x02\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x87\x85\x81\x81\x10a\x02UWa\x02Ua\x0F\x15V[\x90P` \x02\x01` \x81\x01\x90a\x02j\x91\x90a\x0EBV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90a\nyV[\x80a\x02\x91\x81a\x0FDV[\x91PPa\x01\xF6V[P`@Q\x7FZ\xE4\x01\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x030\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x86\x90\x86\x90`\x04\x01a\x0F\xECV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03\x95\x91\x90\x81\x01\x90a\x12'V[PPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[a\x04'\x81Ga\x0BNV[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\xABW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x058W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\\\x91\x90a\x13\x19V[`@Q\x7F.\x1A}M\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xFBW=`\0\x80>=`\0\xFD[PPPPa\x06\t\x82Ga\x0BNV[PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06|W`@Q\x7F\x93?\xE5/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x07\x01W`@Q\x7F\x8Cn]q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x07\x10\x83\x85\x01\x85a\x14\x0EV[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x07\xB6Wa\x07\xA4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x84\x81Q\x81\x10a\x07wWa\x07wa\x0F\x15V[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\ny\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a\x07\xAE\x81a\x0FDV[\x91PPa\x07\x17V[P`@Q\x7FZ\xE4\x01\xDC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZ\xE4\x01\xDC\x90a\x08K\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x85\x90`\x04\x01a\x14\xD0V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\xB0\x91\x90\x81\x01\x90a\x12'V[P`\0[\x86\x81\x10\x15a\t~W`\0\x88\x88\x83\x81\x81\x10a\x08\xD0Wa\x08\xD0a\x0F\x15V[\x90P` \x02\x81\x01\x90a\x08\xE2\x91\x90a\x15\x8EV[a\x08\xEB\x90a\x17qV[\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\tiW`\0\x82`@\x01Q\x82\x81Q\x81\x10a\t\x15Wa\t\x15a\x0F\x15V[` \x02` \x01\x01Q\x90Pa\tV\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0B\xC8\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x80a\ta\x81a\x0FDV[\x91PPa\x08\xF0V[PP\x80\x80a\tv\x90a\x0FDV[\x91PPa\x08\xB4V[PPPPPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\n\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\0`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0E`$\x82\x01R\x7FAPPROVE_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[PPPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x0B\xC3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FETH_TRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x0C}W`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0C=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CBV[``\x91P[PP\x90P\x80a\x0BHW`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xC3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0BHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xEAV[`\0\x80\x83`\x1F\x84\x01\x12a\rzW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x92W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\r\xADW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\r\xCAW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xE2W`\0\x80\xFD[a\r\xEE\x88\x83\x89\x01a\rhV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x0E\x07W`\0\x80\xFD[Pa\x0E\x14\x87\x82\x88\x01a\rhV[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04'W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0ETW`\0\x80\xFD[\x815a\x0E_\x81a\x0E V[\x93\x92PPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x0E~W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x96W`\0\x80\xFD[a\x0E\xA2\x89\x83\x8A\x01a\rhV[\x90\x97P\x95P` \x88\x015\x91Pa\x0E\xB7\x82a\x0E V[\x90\x93P`@\x87\x015\x90\x80\x82\x11\x15a\x0E\xCDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x0E\xE1W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0E\xF0W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x0F\x02W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0F\x9CW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0`@\x82\x01\x85\x83R` `@\x81\x85\x01R\x81\x85\x83R``\x85\x01\x90P``\x86`\x05\x1B\x86\x01\x01\x92P\x86`\0[\x87\x81\x10\x15a\x10\xC1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x83R\x815\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x8A6\x03\x01\x81\x12a\x10wW`\0\x80\xFD[\x89\x01\x84\x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x93W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x10\xA2W`\0\x80\xFD[a\x10\xAD\x87\x82\x84a\x0F\xA3V[\x96PPP\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a\x10\x16V[P\x92\x98\x97PPPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11!Wa\x11!a\x10\xCFV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11!Wa\x11!a\x10\xCFV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x11\x91Wa\x11\x91a\x10\xCFV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xB3Wa\x11\xB3a\x10\xCFV[P`\x05\x1B` \x01\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\xD7Wa\x11\xD7a\x10\xCFV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0[\x83\x81\x10\x15a\x12\x1EW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12\x06V[PP`\0\x91\x01RV[`\0` \x80\x83\x85\x03\x12\x15a\x12:W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12RW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x12fW`\0\x80\xFD[\x81Qa\x12ya\x12t\x82a\x11\x99V[a\x11JV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a\x12\x98W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15a\x13\x0CW\x80Q\x85\x81\x11\x15a\x12\xB4W`\0\x80\x81\xFD[\x86\x01`?\x81\x01\x8B\x13a\x12\xC6W`\0\x80\x81\xFD[\x87\x81\x01Q`@a\x12\xD8a\x12t\x83a\x11\xBDV[\x82\x81R\x8D\x82\x84\x86\x01\x01\x11\x15a\x12\xEDW`\0\x80\x81\xFD[a\x12\xFC\x83\x8C\x83\x01\x84\x87\x01a\x12\x03V[\x86RPPP\x91\x86\x01\x91\x86\x01a\x12\x9CV[P\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x13+W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x13CW`\0\x80\xFD[\x815a\x13Qa\x12t\x82a\x11\xBDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x13fW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x13\x94W`\0\x80\xFD[\x815` a\x13\xA4a\x12t\x83a\x11\x99V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x13\xC3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x14\x03W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xE7W`\0\x80\x81\xFD[a\x13\xF5\x89\x86\x83\x8B\x01\x01a\x132V[\x84RP\x91\x83\x01\x91\x83\x01a\x13\xC7V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14!W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x149W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x14MW`\0\x80\xFD[\x815` a\x14]a\x12t\x83a\x11\x99V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x14|W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x14\xA3W\x855a\x14\x94\x81a\x0E V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x14\x81V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x14\xB9W`\0\x80\xFD[Pa\x14\xC6\x85\x82\x86\x01a\x13\x83V[\x91PP\x92P\x92\x90PV[`\0`@\x82\x01\x84\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P``\x81`\x05\x1B\x87\x01\x01\x93P\x82\x87\x01`\0[\x82\x81\x10\x15a\x15\x80W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x84R\x81Q\x80Q\x80\x88Ra\x15C\x81\x88\x8A\x01\x89\x85\x01a\x12\x03V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x96\x90\x96\x01\x85\x01\x95P\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x14\xFEV[P\x93\x98\x97PPPPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF!\x836\x03\x01\x81\x12a\x15\xC2W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0`\xC0\x82\x84\x03\x12\x15a\x15\xDEW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x16\x02Wa\x16\x02a\x10\xCFV[\x81`@R\x82\x93P\x845\x91Pa\x16\x16\x82a\x0E V[\x90\x82R` \x84\x015\x90a\x16(\x82a\x0E V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x16R\x82a\x0E V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x16lW`\0\x80\xFD[Pa\x16y\x85\x82\x86\x01a\x132V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x16\x98W`\0\x80\xFD[a\x16\xA0a\x10\xFEV[\x90P\x815a\x16\xAD\x81a\x0E V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x16\xDCW`\0\x80\xFD[\x815` a\x16\xECa\x12t\x83a\x11\x99V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x17\x0BW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x17dW\x81\x81\x8A\x03\x12\x15a\x17'W`\0\x80\x81\xFD[a\x17/a\x10\xFEV[\x815a\x17:\x81a\x0E V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x17S\x81a\x0E V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x17\x0FV[P\x90\x97\x96PPPPPPPV[`\0`\xE0\x826\x03\x12\x15a\x17\x83W`\0\x80\xFD[a\x17\x8Ba\x11'V[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xA3W`\0\x80\xFD[a\x17\xAF6\x83\x87\x01a\x15\xCCV[\x83Ra\x17\xBE6` \x87\x01a\x16\x86V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x17\xD7W`\0\x80\xFD[a\x17\xE36\x83\x87\x01a\x16\xCBV[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x17\xFCW`\0\x80\xFD[Pa\x18\t6\x82\x86\x01a\x132V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V\xFE\xA2dipfsX\"\x12 \x86Fa\x13\xE8\x06\x0Bz\x84\x07\x13qz+\x14\xC7\xF0D\xD0\xE4\x90x\xAFl\xE43+F\x04\xC5hzdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SWAPROUTER02EXECUTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                SWAPROUTER02EXECUTOR_ABI.clone(),
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SwapRouter02Executor<M>
    {
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
        Hash,
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
        Hash,
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
        Hash,
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
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <CallerNotWhitelisted as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CallerNotWhitelisted(decoded));
            }
            if let Ok(decoded) =
                <MsgSenderNotReactor as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MsgSenderNotReactor(decoded));
            }
            if let Ok(decoded) =
                <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
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
                    == <CallerNotWhitelisted as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <MsgSenderNotReactor as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NativeTransferFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SwapRouter02ExecutorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallerNotWhitelisted(element) => ::core::fmt::Display::fmt(element, f),
                Self::MsgSenderNotReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReactorCallback(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnwrapWETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnwrapWETH(decoded));
            }
            if let Ok(decoded) = <WithdrawETHCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SwapRouter02ExecutorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Multicall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReactorCallback(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnwrapWETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
