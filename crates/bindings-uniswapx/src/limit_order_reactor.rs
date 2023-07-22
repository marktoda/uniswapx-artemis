pub use limit_order_reactor::*;
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
pub mod limit_order_reactor {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_permit2"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_protocolFeeOwner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DIRECT_FILL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DIRECT_FILL"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IReactorCallback",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("execute"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct SignedOrder"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fillContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IReactorCallback",),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeBatch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeBatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("orders"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct SignedOrder[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fillContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IReactorCallback",),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feeController"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IProtocolFeeController",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit2"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("permit2"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProtocolFeeController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProtocolFeeController",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_newFeeController"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Fill"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Fill"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("orderHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("filler"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("swapper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolFeeControllerSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProtocolFeeControllerSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldFeeController"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newFeeController"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DeadlinePassed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DeadlinePassed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DuplicateFeeOutput"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DuplicateFeeOutput"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("duplicateToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeTooLarge"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FeeTooLarge"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientEth"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientEth"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
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
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidFeeToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("feeToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReactor"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidReactor"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeTransferFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NativeTransferFailed",),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIMITORDERREACTOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0,\t8\x03\x80b\0,\t\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xB8V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x84\x92\x84\x92\x83\x92\x83\x92\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3PP`\x01`\x02UP`\x01`\x01`\xA0\x1B\x03\x16`\x80RPb\0\0\xF7\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xB5W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xCCW`\0\x80\xFD[\x82Qb\0\0\xD9\x81b\0\0\x9FV[` \x84\x01Q\x90\x92Pb\0\0\xEC\x81b\0\0\x9FV[\x80\x91PP\x92P\x92\x90PV[`\x80Qa*\xE9b\0\x01 `\09`\0\x81\x81`\xA7\x01R\x81\x81a\x0F\xE8\x01Ra\x12Z\x01Ra*\xE9`\0\xF3\xFE`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80co\x1D_Q\x11a\0NW\x80co\x1D_Q\x14a\x01?W\x80c\x8D\xA5\xCB[\x14a\x01RW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x7FW\x80c\xFC\xCB\xCA\xAF\x14a\x01\x9FW`\0\x80\xFD[\x80c\x05\xAF\xC9w\x14a\0\x80W\x80c\x12&\x1E\xE7\x14a\0\x95W\x80c-w\x13\x89\x14a\0\xF2W\x80ci\x99\xB3w\x14a\x01\x12W[`\0\x80\xFD[a\0\x93a\0\x8E6`\x04a\x1D\x96V[a\x01\xB4V[\0[4\x80\x15a\0\xA1W`\0\x80\xFD[Pa\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xFEW`\0\x80\xFD[Pa\0\x93a\x01\r6`\x04a\x1E\x19V[a\x02\xB6V[4\x80\x15a\x01\x1EW`\0\x80\xFD[P`\x01Ta\0\xC9\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\0\x93a\x01M6`\x04a\x1E6V[a\x03\xC2V[4\x80\x15a\x01^W`\0\x80\xFD[P`\0Ta\0\xC9\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\x8BW`\0\x80\xFD[Pa\0\x93a\x01\x9A6`\x04a\x1E\x19V[a\x05\x1EV[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\0\xC9`\x01\x81V[a\x01\xBCa\x06\x0FV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x01\xD3W\x90PP\x90Pa\x02{\x85a\x06\x80V[\x81`\0\x81Q\x81\x10a\x02\x8EWa\x02\x8Ea\x1F\x17V[` \x02` \x01\x01\x81\x90RPa\x02\xA5\x81\x85\x85\x85a\x07\x8DV[Pa\x02\xB0`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x03\xCAa\x06\x0FV[`\0\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xE5Wa\x03\xE5a\x1E\xE8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xA0W\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x04\x03W\x90P[P\x90P`\0[\x85\x81\x10\x15a\x04\xFFWa\x04\xDA\x87\x87\x83\x81\x81\x10a\x04\xC3Wa\x04\xC3a\x1F\x17V[\x90P` \x02\x81\x01\x90a\x04\xD5\x91\x90a\x1FFV[a\x06\x80V[\x82\x82\x81Q\x81\x10a\x04\xECWa\x04\xECa\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xA6V[Pa\x05\x0C\x81\x85\x85\x85a\x07\x8DV[Pa\x05\x17`\x01`\x02UV[PPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05\x9FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x033V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\x06zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x033V[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\x06\xF9\x83\x80a\x1F\x84V[\x81\x01\x90a\x07\x06\x91\x90a\"&V[\x90P`@Q\x80`\xA0\x01`@R\x80\x82`\0\x01Q\x81R` \x01\x82` \x01Q\x81R` \x01\x82`@\x01Q\x81R` \x01\x84\x80` \x01\x90a\x07A\x91\x90a\x1F\x84V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x07\x84\x83a\tUV[\x90R\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x01\x14`\0[\x85Q\x81\x10\x15a\x08\x97W`\0\x86\x82\x81Q\x81\x10a\x07\xC7Wa\x07\xC7a\x1F\x17V[` \x02` \x01\x01Q\x90Pa\x07\xDA\x81a\n0V[a\x07\xE4\x813a\x0E\xAAV[a\x07\xF9\x81\x84a\x07\xF3W\x87a\x0F\xE6V[3a\x0F\xE6V[\x80`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x84\x81Q\x81\x10a\x08AWa\x08Aa\x1F\x17V[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x84`\0\x01Q`@\x01Q`@Qa\x08\x86\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4P`\x01\x01a\x07\xAAV[P\x80\x15a\x08\xACWa\x08\xA7\x85a\x11\xEFV[a\x05\x17V[`\0a\x08\xB7\x86a\x12\xCDV[`@Q\x7F\x99C\xFA\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x99C\xFA\x89\x90a\t\x12\x90\x89\x903\x90\x89\x90\x89\x90`\x04\x01a%\x17V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t@W=`\0\x80>=`\0\xFD[PPPPa\tM\x81a\x15\x93V[PPPPPPV[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a*'`\x8D\x919`@Q\x80``\x01`@R\x80`;\x81R` \x01a)\xBE`;\x919`@Q` \x01a\t\x9A\x92\x91\x90a%\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\t\xBD\x83`\0\x01Qa\x16VV[` \x80\x85\x01Q\x80Q\x91\x01Q`@\x86\x01Qa\t\xD6\x90a\x16\xF0V[`@\x80Q` \x81\x01\x96\x90\x96R\x85\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\nPWPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\n\xA7\x90\x85\x90`\x04\x01a&\xEDV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0B\n\x91\x90\x81\x01\x90a'\0V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x0B\"\x82\x84a'\xFFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B:Wa\x0B:a\x1E\xE8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xA3W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x0BXW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0B\xF4W\x85`@\x01Q\x81\x81Q\x81\x10a\x0B\xC7Wa\x0B\xC7a\x1F\x17V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0B\xE1Wa\x0B\xE1a\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0B\xA9V[P`\0[\x82\x81\x10\x15a\x0E\x9BW`\0\x85\x82\x81Q\x81\x10a\x0C\x14Wa\x0C\x14a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x0C\xD2W\x86\x81\x81Q\x81\x10a\x0C;Wa\x0C;a\x1F\x17V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\xCAW\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x033V[`\x01\x01a\x0C!V[P`\0\x80[\x86\x81\x10\x15a\rWW`\0\x89`@\x01Q\x82\x81Q\x81\x10a\x0C\xF7Wa\x0C\xF7a\x1F\x17V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\rNW` \x81\x01Qa\rK\x90\x84a'\xFFV[\x92P[P`\x01\x01a\x0C\xD7V[P\x81Q` \x89\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\r\x94W` \x80\x89\x01Q\x01Qa\r\x91\x90\x82a'\xFFV[\x90P[\x80`\0\x03a\r\xE9W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x033V[a\r\xF7\x81`\x05a'\x10a\x17\x8EV[\x82` \x01Q\x11\x15a\x0EjW\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x033V[\x81\x84a\x0Ev\x85\x89a'\xFFV[\x81Q\x81\x10a\x0E\x86Wa\x0E\x86a\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0B\xF8V[P`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x0E\xFBW`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q``\x01QB\x11\x15a\x0F:W`@Q\x7Fp\xF6\\\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x0F\xE2W\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x0F\xB6\x90\x84\x90\x86\x90`\x04\x01a(\x12V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xCEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\tMW=`\0\x80>=`\0\xFD[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x10\xA6\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[a\x10\xB0\x85\x85a\x17\xCAV[\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a*'`\x8D\x919`@Q\x80``\x01`@R\x80`;\x81R` \x01a)\xBE`;\x919`@Q` \x01a\x11\x01\x92\x91\x90a%\xFAV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R``\x83\x01\x90\x91R`.\x80\x83R\x90\x91\x90a)\xF9` \x83\x019`@Q` \x01a\x11T\x92\x91\x90a(IV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x11\xC1\x96\x95\x94\x93\x92`\x04\x01a(\xA4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xDBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\tMW=`\0\x80>=`\0\xFD[`\0[\x81Q\x81\x10\x15a\x12\xB7W`\0\x82\x82\x81Q\x81\x10a\x12\x0FWa\x12\x0Fa\x1F\x17V[` \x02` \x01\x01Q\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\x12\xADW`\0\x82`@\x01Q\x82\x81Q\x81\x10a\x12AWa\x12Aa\x1F\x17V[` \x02` \x01\x01Q\x90Pa\x12\xA4\x81`@\x01Q\x82` \x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x11\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x12\x1CV[PP`\x01\x01a\x11\xF2V[PG\x15a\x12\xCAWa\x12\xCA`\x003Ga\x19\x8EV[PV[```\0\x80[\x83Q\x81\x10\x15a\x13\x0CW\x83\x81\x81Q\x81\x10a\x12\xEEWa\x12\xEEa\x1F\x17V[` \x02` \x01\x01Q`@\x01QQ\x82\x01\x91P\x80\x80`\x01\x01\x91PPa\x12\xD3V[P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13&Wa\x13&a\x1E\xE8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x8FW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x13DW\x90P[P\x91PP`\0\x80[\x83Q\x81\x10\x15a\x15\x8BW`\0\x84\x82\x81Q\x81\x10a\x13\xB4Wa\x13\xB4a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\x15\x81W`\0\x82`@\x01Q\x82\x81Q\x81\x10a\x13\xE6Wa\x13\xE6a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0\x80[\x86\x81\x10\x15a\x14\xC2W`\0\x88\x82\x81Q\x81\x10a\x14\x10Wa\x14\x10a\x1F\x17V[` \x02` \x01\x01Q\x90P\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\x14\x90WP\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x15a\x14\xB9W`\x01\x92P\x83` \x01Q\x81`@\x01\x81\x81Qa\x14\xAF\x91\x90a'\xFFV[\x90RPa\x14\xC2\x90PV[P`\x01\x01a\x13\xF4V[P\x80a\x15wW`@\x82\x01Q\x82Q`\0\x91a\x14\xF2\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a\x1AiV[\x90P`@Q\x80``\x01`@R\x80\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84` \x01Q\x83a\x15O\x91\x90a'\xFFV[\x81RP\x88\x88\x81Q\x81\x10a\x15dWa\x15da\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x90\x95\x01\x94[PP`\x01\x01a\x13\xC1V[PP`\x01\x01a\x13\x97V[P\x81R\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x0F\xE2W`\0\x82\x82\x81Q\x81\x10a\x15\xB3Wa\x15\xB3a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0a\x15\xF0\x82`\0\x01Q\x83` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1Ai\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x81`@\x01Q\x81\x10\x15a\x16AW\x80\x82`@\x01Q`@Q\x7F,\x19\xB8\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x033\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[PP\x80\x80a\x16N\x90a)lV[\x91PPa\x15\x96V[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a*'`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\n\x13\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x10Wa\x17\x10a\x1E\xE8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17:W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x17\x7FW`\0a\x17m\x85\x83\x81Q\x81\x10a\x17`Wa\x17`a\x1F\x17V[` \x02` \x01\x01Qa\x1B;V[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a\x17@V[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x17\xC3W`\0\x80\xFD[P\x91\x02\x04\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x82R\x83\x81\x01Q\x81\x01Q\x90\x82\x01R[\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x18\xCCW`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x8BV[``\x91P[PP\x90P\x80a\x18\xC6W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x02\xB0V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c6\xC7\x85\x163\x85a\x18\xF3\x86a\x1B\xA2V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x82\x16`D\x82\x01R\x90\x87\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19pW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x84W=`\0\x80>=`\0\xFD[PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1ACW`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1A\x03W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A\x08V[``\x91P[PP\x90P\x80a\x02\xB0W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Ads\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x1CLV[PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1A\xA4WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x161a\x18\x0BV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B4\x91\x90a)\xA4V[\x93\x92PPPV[`\0`@Q\x80``\x01`@R\x80`;\x81R` \x01a)\xBE`;\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q\x81Q\x95\x86\x01\x94\x90\x94Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x90\x85\x01R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01a\n\x13V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1CHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01R\x7F60 bits\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x033V[P\x90V[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x02\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x033V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xCAW`\0\x80\xFD[\x805a\x1DH\x81a\x1D\x1BV[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1D_W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DwW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D\x8FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1D\xACW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D\xC4W`\0\x80\xFD[\x90\x86\x01\x90`@\x82\x89\x03\x12\x15a\x1D\xD8W`\0\x80\xFD[\x90\x94P` \x86\x015\x90a\x1D\xEA\x82a\x1D\x1BV[\x90\x93P`@\x86\x015\x90\x80\x82\x11\x15a\x1E\0W`\0\x80\xFD[Pa\x1E\r\x87\x82\x88\x01a\x1DMV[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\x1E+W`\0\x80\xFD[\x815a\x1B4\x81a\x1D\x1BV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1ENW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1EfW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1EzW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1E\x89W`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\x9EW`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\x1E\xB4` \x89\x01a\x1D=V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1E\xCAW`\0\x80\xFD[Pa\x1E\xD7\x88\x82\x89\x01a\x1DMV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a\x1FzW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x1F\xB9W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\xD4W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1D\x8FW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \x0CWa \x0Ca\x1E\xE8V[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \x0CWa \x0Ca\x1E\xE8V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a |Wa |a\x1E\xE8V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a \x95W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xAFWa \xAFa\x1E\xE8V[a \xE0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a 5V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a \xF5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a!$W`\0\x80\xFD[a!,a\x1F\xE9V[\x90P\x815a!9\x81a\x1D\x1BV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a!qWa!qa\x1E\xE8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a!\x8CW`\0\x80\xFD[\x815` a!\xA1a!\x9C\x83a!WV[a 5V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a!\xC0W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\"\x19W\x81\x81\x8A\x03\x12\x15a!\xDCW`\0\x80\x81\xFD[a!\xE4a\x1F\xE9V[\x815a!\xEF\x81a\x1D\x1BV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\"\x08\x81a\x1D\x1BV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a!\xC4V[P\x90\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\"8W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"PW`\0\x80\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a\"dW`\0\x80\xFD[a\"la\x1F\xE9V[\x825\x82\x81\x11\x15a\"{W`\0\x80\xFD[\x83\x01`\xC0\x81\x88\x03\x12\x15a\"\x8DW`\0\x80\xFD[a\"\x95a \x12V[\x815a\"\xA0\x81a\x1D\x1BV[\x81R` \x82\x015a\"\xB0\x81a\x1D\x1BV[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a\"\xD9\x81a\x1D\x1BV[`\x80\x82\x01R`\xA0\x82\x015\x84\x81\x11\x15a\"\xF0W`\0\x80\xFD[a\"\xFC\x89\x82\x85\x01a \x84V[`\xA0\x83\x01RP\x82RPa#\x12\x86` \x85\x01a!\x12V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a#)W`\0\x80\xFD[a#5\x87\x82\x86\x01a!{V[`@\x83\x01RP\x95\x94PPPPPV[`\0[\x83\x81\x10\x15a#_W\x81\x81\x01Q\x83\x82\x01R` \x01a#GV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra#\x80\x81` \x86\x01` \x86\x01a#DV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$\x11W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a#\xC6V[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra$\x90a\x01\xA0\x85\x01\x82a#hV[\x90P` \x83\x01Qa$\xCE` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra$\xE6\x82\x82a#\xB2V[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra%\0\x82\x82a#hV[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[`\0``\x82\x01``\x83R\x80\x87Q\x80\x83R`\x80\x85\x01\x91P`\x80\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x8A\x01`\0[\x83\x81\x10\x15a%\x8CW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x87\x03\x01\x85Ra%z\x86\x83Qa$\x1CV[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a%@V[PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x81\x87\x01R\x85\x84\x03`@\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x97\x96PPPPPPPV[\x7FLimitOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`\x1A\x82\x01R\x7Fuint256 inputAmount,\0\0\0\0\0\0\0\0\0\0\0\0`-\x82\x01R\x7FOutputToken[] outputs)\0\0\0\0\0\0\0\0\0\0`A\x82\x01R`\0\x83Qa&\xCA\x81`W\x85\x01` \x88\x01a#DV[\x83Q\x90\x83\x01\x90a&\xE1\x81`W\x84\x01` \x88\x01a#DV[\x01`W\x01\x94\x93PPPPV[` \x81R`\0a\x1B4` \x83\x01\x84a$\x1CV[`\0` \x80\x83\x85\x03\x12\x15a'\x13W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'*W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a';W`\0\x80\xFD[\x80Qa'Ia!\x9C\x82a!WV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a'hW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a'\xC4W\x80\x85\x8A\x03\x12\x15a'\x85W`\0\x80\x81\xFD[a'\x8Da\x1F\xE9V[\x85Qa'\x98\x81a\x1D\x1BV[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa'\xB1\x81a\x1D\x1BV[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a'mV[P\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x18\x0BWa\x18\x0Ba'\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a(A`@\x83\x01\x84a$\x1CV[\x94\x93PPPPV[\x7FLimitOrder witness)\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa(\x81\x81`\x13\x85\x01` \x88\x01a#DV[\x83Q\x90\x83\x01\x90a(\x98\x81`\x13\x84\x01` \x88\x01a#DV[\x01`\x13\x01\x94\x93PPPPV[`\0a\x01@a(\xD4\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra)\x15`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra)J\x81\x84\x01\x86a#hV[\x90P\x82\x81\x03a\x01 \x84\x01Ra)_\x81\x85a#hV[\x99\x98PPPPPPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a)\x9DWa)\x9Da'\xD0V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a)\xB6W`\0\x80\xFD[PQ\x91\x90PV\xFEOutputToken(address token,uint256 amount,address recipient)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)\xA2dipfsX\"\x12 \xF2\xF0c\x84\xC0Y0&C\xDD\x81Fkm\xFC\x1A\x9Ck]\xFEb\x05G\xA51\x84\xCE\xD8\xC3F\xB2\x7FdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LIMITORDERREACTOR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80co\x1D_Q\x11a\0NW\x80co\x1D_Q\x14a\x01?W\x80c\x8D\xA5\xCB[\x14a\x01RW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x7FW\x80c\xFC\xCB\xCA\xAF\x14a\x01\x9FW`\0\x80\xFD[\x80c\x05\xAF\xC9w\x14a\0\x80W\x80c\x12&\x1E\xE7\x14a\0\x95W\x80c-w\x13\x89\x14a\0\xF2W\x80ci\x99\xB3w\x14a\x01\x12W[`\0\x80\xFD[a\0\x93a\0\x8E6`\x04a\x1D\x96V[a\x01\xB4V[\0[4\x80\x15a\0\xA1W`\0\x80\xFD[Pa\0\xC9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xFEW`\0\x80\xFD[Pa\0\x93a\x01\r6`\x04a\x1E\x19V[a\x02\xB6V[4\x80\x15a\x01\x1EW`\0\x80\xFD[P`\x01Ta\0\xC9\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\0\x93a\x01M6`\x04a\x1E6V[a\x03\xC2V[4\x80\x15a\x01^W`\0\x80\xFD[P`\0Ta\0\xC9\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\x8BW`\0\x80\xFD[Pa\0\x93a\x01\x9A6`\x04a\x1E\x19V[a\x05\x1EV[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\0\xC9`\x01\x81V[a\x01\xBCa\x06\x0FV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x01\xD3W\x90PP\x90Pa\x02{\x85a\x06\x80V[\x81`\0\x81Q\x81\x10a\x02\x8EWa\x02\x8Ea\x1F\x17V[` \x02` \x01\x01\x81\x90RPa\x02\xA5\x81\x85\x85\x85a\x07\x8DV[Pa\x02\xB0`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03<W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x03\xCAa\x06\x0FV[`\0\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xE5Wa\x03\xE5a\x1E\xE8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xA0W\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x04\x03W\x90P[P\x90P`\0[\x85\x81\x10\x15a\x04\xFFWa\x04\xDA\x87\x87\x83\x81\x81\x10a\x04\xC3Wa\x04\xC3a\x1F\x17V[\x90P` \x02\x81\x01\x90a\x04\xD5\x91\x90a\x1FFV[a\x06\x80V[\x82\x82\x81Q\x81\x10a\x04\xECWa\x04\xECa\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xA6V[Pa\x05\x0C\x81\x85\x85\x85a\x07\x8DV[Pa\x05\x17`\x01`\x02UV[PPPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05\x9FW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x033V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\x06zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x033V[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\x06\xF9\x83\x80a\x1F\x84V[\x81\x01\x90a\x07\x06\x91\x90a\"&V[\x90P`@Q\x80`\xA0\x01`@R\x80\x82`\0\x01Q\x81R` \x01\x82` \x01Q\x81R` \x01\x82`@\x01Q\x81R` \x01\x84\x80` \x01\x90a\x07A\x91\x90a\x1F\x84V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x07\x84\x83a\tUV[\x90R\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x01\x14`\0[\x85Q\x81\x10\x15a\x08\x97W`\0\x86\x82\x81Q\x81\x10a\x07\xC7Wa\x07\xC7a\x1F\x17V[` \x02` \x01\x01Q\x90Pa\x07\xDA\x81a\n0V[a\x07\xE4\x813a\x0E\xAAV[a\x07\xF9\x81\x84a\x07\xF3W\x87a\x0F\xE6V[3a\x0F\xE6V[\x80`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x84\x81Q\x81\x10a\x08AWa\x08Aa\x1F\x17V[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x84`\0\x01Q`@\x01Q`@Qa\x08\x86\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4P`\x01\x01a\x07\xAAV[P\x80\x15a\x08\xACWa\x08\xA7\x85a\x11\xEFV[a\x05\x17V[`\0a\x08\xB7\x86a\x12\xCDV[`@Q\x7F\x99C\xFA\x89\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x99C\xFA\x89\x90a\t\x12\x90\x89\x903\x90\x89\x90\x89\x90`\x04\x01a%\x17V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t@W=`\0\x80>=`\0\xFD[PPPPa\tM\x81a\x15\x93V[PPPPPPV[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a*'`\x8D\x919`@Q\x80``\x01`@R\x80`;\x81R` \x01a)\xBE`;\x919`@Q` \x01a\t\x9A\x92\x91\x90a%\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\t\xBD\x83`\0\x01Qa\x16VV[` \x80\x85\x01Q\x80Q\x91\x01Q`@\x86\x01Qa\t\xD6\x90a\x16\xF0V[`@\x80Q` \x81\x01\x96\x90\x96R\x85\x01\x93\x90\x93Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\nPWPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\n\xA7\x90\x85\x90`\x04\x01a&\xEDV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0B\n\x91\x90\x81\x01\x90a'\0V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x0B\"\x82\x84a'\xFFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B:Wa\x0B:a\x1E\xE8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xA3W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x0BXW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0B\xF4W\x85`@\x01Q\x81\x81Q\x81\x10a\x0B\xC7Wa\x0B\xC7a\x1F\x17V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0B\xE1Wa\x0B\xE1a\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0B\xA9V[P`\0[\x82\x81\x10\x15a\x0E\x9BW`\0\x85\x82\x81Q\x81\x10a\x0C\x14Wa\x0C\x14a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x0C\xD2W\x86\x81\x81Q\x81\x10a\x0C;Wa\x0C;a\x1F\x17V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0C\xCAW\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x033V[`\x01\x01a\x0C!V[P`\0\x80[\x86\x81\x10\x15a\rWW`\0\x89`@\x01Q\x82\x81Q\x81\x10a\x0C\xF7Wa\x0C\xF7a\x1F\x17V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\rNW` \x81\x01Qa\rK\x90\x84a'\xFFV[\x92P[P`\x01\x01a\x0C\xD7V[P\x81Q` \x89\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\r\x94W` \x80\x89\x01Q\x01Qa\r\x91\x90\x82a'\xFFV[\x90P[\x80`\0\x03a\r\xE9W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x033V[a\r\xF7\x81`\x05a'\x10a\x17\x8EV[\x82` \x01Q\x11\x15a\x0EjW\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x033V[\x81\x84a\x0Ev\x85\x89a'\xFFV[\x81Q\x81\x10a\x0E\x86Wa\x0E\x86a\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0B\xF8V[P`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x0E\xFBW`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q``\x01QB\x11\x15a\x0F:W`@Q\x7Fp\xF6\\\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x0F\xE2W\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x0F\xB6\x90\x84\x90\x86\x90`\x04\x01a(\x12V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\xCEW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\tMW=`\0\x80>=`\0\xFD[PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x10\xA6\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[a\x10\xB0\x85\x85a\x17\xCAV[\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a*'`\x8D\x919`@Q\x80``\x01`@R\x80`;\x81R` \x01a)\xBE`;\x919`@Q` \x01a\x11\x01\x92\x91\x90a%\xFAV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R``\x83\x01\x90\x91R`.\x80\x83R\x90\x91\x90a)\xF9` \x83\x019`@Q` \x01a\x11T\x92\x91\x90a(IV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x11\xC1\x96\x95\x94\x93\x92`\x04\x01a(\xA4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\xDBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\tMW=`\0\x80>=`\0\xFD[`\0[\x81Q\x81\x10\x15a\x12\xB7W`\0\x82\x82\x81Q\x81\x10a\x12\x0FWa\x12\x0Fa\x1F\x17V[` \x02` \x01\x01Q\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\x12\xADW`\0\x82`@\x01Q\x82\x81Q\x81\x10a\x12AWa\x12Aa\x1F\x17V[` \x02` \x01\x01Q\x90Pa\x12\xA4\x81`@\x01Q\x82` \x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x11\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x12\x1CV[PP`\x01\x01a\x11\xF2V[PG\x15a\x12\xCAWa\x12\xCA`\x003Ga\x19\x8EV[PV[```\0\x80[\x83Q\x81\x10\x15a\x13\x0CW\x83\x81\x81Q\x81\x10a\x12\xEEWa\x12\xEEa\x1F\x17V[` \x02` \x01\x01Q`@\x01QQ\x82\x01\x91P\x80\x80`\x01\x01\x91PPa\x12\xD3V[P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13&Wa\x13&a\x1E\xE8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x8FW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x13DW\x90P[P\x91PP`\0\x80[\x83Q\x81\x10\x15a\x15\x8BW`\0\x84\x82\x81Q\x81\x10a\x13\xB4Wa\x13\xB4a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0[\x81`@\x01QQ\x81\x10\x15a\x15\x81W`\0\x82`@\x01Q\x82\x81Q\x81\x10a\x13\xE6Wa\x13\xE6a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0\x80[\x86\x81\x10\x15a\x14\xC2W`\0\x88\x82\x81Q\x81\x10a\x14\x10Wa\x14\x10a\x1F\x17V[` \x02` \x01\x01Q\x90P\x83`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\x14\x90WP\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[\x15a\x14\xB9W`\x01\x92P\x83` \x01Q\x81`@\x01\x81\x81Qa\x14\xAF\x91\x90a'\xFFV[\x90RPa\x14\xC2\x90PV[P`\x01\x01a\x13\xF4V[P\x80a\x15wW`@\x82\x01Q\x82Q`\0\x91a\x14\xF2\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a\x1AiV[\x90P`@Q\x80``\x01`@R\x80\x84`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84` \x01Q\x83a\x15O\x91\x90a'\xFFV[\x81RP\x88\x88\x81Q\x81\x10a\x15dWa\x15da\x1F\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x90\x95\x01\x94[PP`\x01\x01a\x13\xC1V[PP`\x01\x01a\x13\x97V[P\x81R\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x0F\xE2W`\0\x82\x82\x81Q\x81\x10a\x15\xB3Wa\x15\xB3a\x1F\x17V[` \x02` \x01\x01Q\x90P`\0a\x15\xF0\x82`\0\x01Q\x83` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1Ai\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x81`@\x01Q\x81\x10\x15a\x16AW\x80\x82`@\x01Q`@Q\x7F,\x19\xB8\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x033\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[PP\x80\x80a\x16N\x90a)lV[\x91PPa\x15\x96V[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a*'`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\n\x13\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x10Wa\x17\x10a\x1E\xE8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x17:W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x17\x7FW`\0a\x17m\x85\x83\x81Q\x81\x10a\x17`Wa\x17`a\x1F\x17V[` \x02` \x01\x01Qa\x1B;V[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a\x17@V[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x17\xC3W`\0\x80\xFD[P\x91\x02\x04\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x82R\x83\x81\x01Q\x81\x01Q\x90\x82\x01R[\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x18\xCCW`\0\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18\x86W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\x8BV[``\x91P[PP\x90P\x80a\x18\xC6W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa\x02\xB0V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c6\xC7\x85\x163\x85a\x18\xF3\x86a\x1B\xA2V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x86\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x82\x16`D\x82\x01R\x90\x87\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19pW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x84W=`\0\x80>=`\0\xFD[PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1ACW`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1A\x03W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A\x08V[``\x91P[PP\x90P\x80a\x02\xB0W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Ads\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x1CLV[PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1A\xA4WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x161a\x18\x0BV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B4\x91\x90a)\xA4V[\x93\x92PPPV[`\0`@Q\x80``\x01`@R\x80`;\x81R` \x01a)\xBE`;\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q\x81Q\x95\x86\x01\x94\x90\x94Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x90\x85\x01R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01a\n\x13V[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1CHW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01R\x7F60 bits\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x033V[P\x90V[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x02\xB0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x033V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xCAW`\0\x80\xFD[\x805a\x1DH\x81a\x1D\x1BV[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1D_W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DwW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1D\x8FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x1D\xACW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D\xC4W`\0\x80\xFD[\x90\x86\x01\x90`@\x82\x89\x03\x12\x15a\x1D\xD8W`\0\x80\xFD[\x90\x94P` \x86\x015\x90a\x1D\xEA\x82a\x1D\x1BV[\x90\x93P`@\x86\x015\x90\x80\x82\x11\x15a\x1E\0W`\0\x80\xFD[Pa\x1E\r\x87\x82\x88\x01a\x1DMV[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a\x1E+W`\0\x80\xFD[\x815a\x1B4\x81a\x1D\x1BV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1ENW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1EfW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1EzW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1E\x89W`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\x9EW`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\x1E\xB4` \x89\x01a\x1D=V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1E\xCAW`\0\x80\xFD[Pa\x1E\xD7\x88\x82\x89\x01a\x1DMV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a\x1FzW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\x1F\xB9W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1F\xD4W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1D\x8FW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \x0CWa \x0Ca\x1E\xE8V[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a \x0CWa \x0Ca\x1E\xE8V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a |Wa |a\x1E\xE8V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a \x95W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xAFWa \xAFa\x1E\xE8V[a \xE0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a 5V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a \xF5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a!$W`\0\x80\xFD[a!,a\x1F\xE9V[\x90P\x815a!9\x81a\x1D\x1BV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a!qWa!qa\x1E\xE8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a!\x8CW`\0\x80\xFD[\x815` a!\xA1a!\x9C\x83a!WV[a 5V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a!\xC0W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\"\x19W\x81\x81\x8A\x03\x12\x15a!\xDCW`\0\x80\x81\xFD[a!\xE4a\x1F\xE9V[\x815a!\xEF\x81a\x1D\x1BV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\"\x08\x81a\x1D\x1BV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a!\xC4V[P\x90\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\"8W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"PW`\0\x80\xFD[\x90\x83\x01\x90`\xA0\x82\x86\x03\x12\x15a\"dW`\0\x80\xFD[a\"la\x1F\xE9V[\x825\x82\x81\x11\x15a\"{W`\0\x80\xFD[\x83\x01`\xC0\x81\x88\x03\x12\x15a\"\x8DW`\0\x80\xFD[a\"\x95a \x12V[\x815a\"\xA0\x81a\x1D\x1BV[\x81R` \x82\x015a\"\xB0\x81a\x1D\x1BV[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a\"\xD9\x81a\x1D\x1BV[`\x80\x82\x01R`\xA0\x82\x015\x84\x81\x11\x15a\"\xF0W`\0\x80\xFD[a\"\xFC\x89\x82\x85\x01a \x84V[`\xA0\x83\x01RP\x82RPa#\x12\x86` \x85\x01a!\x12V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a#)W`\0\x80\xFD[a#5\x87\x82\x86\x01a!{V[`@\x83\x01RP\x95\x94PPPPPV[`\0[\x83\x81\x10\x15a#_W\x81\x81\x01Q\x83\x82\x01R` \x01a#GV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra#\x80\x81` \x86\x01` \x86\x01a#DV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$\x11W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a#\xC6V[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra$\x90a\x01\xA0\x85\x01\x82a#hV[\x90P` \x83\x01Qa$\xCE` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra$\xE6\x82\x82a#\xB2V[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra%\0\x82\x82a#hV[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[`\0``\x82\x01``\x83R\x80\x87Q\x80\x83R`\x80\x85\x01\x91P`\x80\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x8A\x01`\0[\x83\x81\x10\x15a%\x8CW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x87\x03\x01\x85Ra%z\x86\x83Qa$\x1CV[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a%@V[PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x81\x87\x01R\x85\x84\x03`@\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x97\x96PPPPPPPV[\x7FLimitOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`\x1A\x82\x01R\x7Fuint256 inputAmount,\0\0\0\0\0\0\0\0\0\0\0\0`-\x82\x01R\x7FOutputToken[] outputs)\0\0\0\0\0\0\0\0\0\0`A\x82\x01R`\0\x83Qa&\xCA\x81`W\x85\x01` \x88\x01a#DV[\x83Q\x90\x83\x01\x90a&\xE1\x81`W\x84\x01` \x88\x01a#DV[\x01`W\x01\x94\x93PPPPV[` \x81R`\0a\x1B4` \x83\x01\x84a$\x1CV[`\0` \x80\x83\x85\x03\x12\x15a'\x13W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'*W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a';W`\0\x80\xFD[\x80Qa'Ia!\x9C\x82a!WV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a'hW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a'\xC4W\x80\x85\x8A\x03\x12\x15a'\x85W`\0\x80\x81\xFD[a'\x8Da\x1F\xE9V[\x85Qa'\x98\x81a\x1D\x1BV[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa'\xB1\x81a\x1D\x1BV[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a'mV[P\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x18\x0BWa\x18\x0Ba'\xD0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a(A`@\x83\x01\x84a$\x1CV[\x94\x93PPPPV[\x7FLimitOrder witness)\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa(\x81\x81`\x13\x85\x01` \x88\x01a#DV[\x83Q\x90\x83\x01\x90a(\x98\x81`\x13\x84\x01` \x88\x01a#DV[\x01`\x13\x01\x94\x93PPPPV[`\0a\x01@a(\xD4\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra)\x15`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra)J\x81\x84\x01\x86a#hV[\x90P\x82\x81\x03a\x01 \x84\x01Ra)_\x81\x85a#hV[\x99\x98PPPPPPPPPV[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a)\x9DWa)\x9Da'\xD0V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a)\xB6W`\0\x80\xFD[PQ\x91\x90PV\xFEOutputToken(address token,uint256 amount,address recipient)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)\xA2dipfsX\"\x12 \xF2\xF0c\x84\xC0Y0&C\xDD\x81Fkm\xFC\x1A\x9Ck]\xFEb\x05G\xA51\x84\xCE\xD8\xC3F\xB2\x7FdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LIMITORDERREACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LimitOrderReactor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LimitOrderReactor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LimitOrderReactor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LimitOrderReactor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LimitOrderReactor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LimitOrderReactor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LimitOrderReactor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LIMITORDERREACTOR_ABI.clone(),
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
                LIMITORDERREACTOR_ABI.clone(),
                LIMITORDERREACTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DIRECT_FILL` (0xfccbcaaf) function
        pub fn direct_fill(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([252, 203, 202, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x05afc977) function
        pub fn execute(
            &self,
            order: SignedOrder,
            fill_contract: ::ethers::core::types::Address,
            fill_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 175, 201, 119], (order, fill_contract, fill_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0x6f1d5f51) function
        pub fn execute_batch(
            &self,
            orders: ::std::vec::Vec<SignedOrder>,
            fill_contract: ::ethers::core::types::Address,
            fill_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 29, 95, 81], (orders, fill_contract, fill_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeController` (0x6999b377) function
        pub fn fee_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([105, 153, 179, 119], ())
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
        ///Calls the contract's `permit2` (0x12261ee7) function
        pub fn permit_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([18, 38, 30, 231], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFeeController` (0x2d771389) function
        pub fn set_protocol_fee_controller(
            &self,
            new_fee_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 119, 19, 137], new_fee_controller)
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
        ///Gets the contract's `Fill` event
        pub fn fill_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FillFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ProtocolFeeControllerSet` event
        pub fn protocol_fee_controller_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProtocolFeeControllerSetFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LimitOrderReactorEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for LimitOrderReactor<M>
    {
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
        Hash,
    )]
    #[etherror(name = "DeadlinePassed", abi = "DeadlinePassed()")]
    pub struct DeadlinePassed;
    ///Custom Error type `DuplicateFeeOutput` with signature `DuplicateFeeOutput(address)` and selector `0xfff08303`
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
    #[etherror(name = "DuplicateFeeOutput", abi = "DuplicateFeeOutput(address)")]
    pub struct DuplicateFeeOutput {
        pub duplicate_token: ::ethers::core::types::Address,
    }
    ///Custom Error type `FeeTooLarge` with signature `FeeTooLarge(address,uint256,address)` and selector `0x82e75656`
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
    #[etherror(name = "FeeTooLarge", abi = "FeeTooLarge(address,uint256,address)")]
    pub struct FeeTooLarge {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Custom Error type `InsufficientEth` with signature `InsufficientEth()` and selector `0xa01a9df6`
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
    #[etherror(name = "InsufficientEth", abi = "InsufficientEth()")]
    pub struct InsufficientEth;
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
    ///Custom Error type `InvalidFeeToken` with signature `InvalidFeeToken(address)` and selector `0xeddf07f5`
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
    #[etherror(name = "InvalidFeeToken", abi = "InvalidFeeToken(address)")]
    pub struct InvalidFeeToken {
        pub fee_token: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidReactor` with signature `InvalidReactor()` and selector `0x4ddf4a64`
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
    #[etherror(name = "InvalidReactor", abi = "InvalidReactor()")]
    pub struct InvalidReactor;
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
    pub enum LimitOrderReactorErrors {
        DeadlinePassed(DeadlinePassed),
        DuplicateFeeOutput(DuplicateFeeOutput),
        FeeTooLarge(FeeTooLarge),
        InsufficientEth(InsufficientEth),
        InsufficientOutput(InsufficientOutput),
        InvalidFeeToken(InvalidFeeToken),
        InvalidReactor(InvalidReactor),
        NativeTransferFailed(NativeTransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LimitOrderReactorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DeadlinePassed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeadlinePassed(decoded));
            }
            if let Ok(decoded) =
                <DuplicateFeeOutput as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DuplicateFeeOutput(decoded));
            }
            if let Ok(decoded) = <FeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeTooLarge(decoded));
            }
            if let Ok(decoded) = <InsufficientEth as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InsufficientEth(decoded));
            }
            if let Ok(decoded) =
                <InsufficientOutput as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientOutput(decoded));
            }
            if let Ok(decoded) = <InvalidFeeToken as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidFeeToken(decoded));
            }
            if let Ok(decoded) = <InvalidReactor as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidReactor(decoded));
            }
            if let Ok(decoded) =
                <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeTransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LimitOrderReactorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeadlinePassed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DuplicateFeeOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooLarge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientEth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidReactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LimitOrderReactorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DeadlinePassed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DuplicateFeeOutput as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FeeTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InsufficientEth as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientOutput as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidFeeToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidReactor as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for LimitOrderReactorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeadlinePassed(element) => ::core::fmt::Display::fmt(element, f),
                Self::DuplicateFeeOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LimitOrderReactorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeadlinePassed> for LimitOrderReactorErrors {
        fn from(value: DeadlinePassed) -> Self {
            Self::DeadlinePassed(value)
        }
    }
    impl ::core::convert::From<DuplicateFeeOutput> for LimitOrderReactorErrors {
        fn from(value: DuplicateFeeOutput) -> Self {
            Self::DuplicateFeeOutput(value)
        }
    }
    impl ::core::convert::From<FeeTooLarge> for LimitOrderReactorErrors {
        fn from(value: FeeTooLarge) -> Self {
            Self::FeeTooLarge(value)
        }
    }
    impl ::core::convert::From<InsufficientEth> for LimitOrderReactorErrors {
        fn from(value: InsufficientEth) -> Self {
            Self::InsufficientEth(value)
        }
    }
    impl ::core::convert::From<InsufficientOutput> for LimitOrderReactorErrors {
        fn from(value: InsufficientOutput) -> Self {
            Self::InsufficientOutput(value)
        }
    }
    impl ::core::convert::From<InvalidFeeToken> for LimitOrderReactorErrors {
        fn from(value: InvalidFeeToken) -> Self {
            Self::InvalidFeeToken(value)
        }
    }
    impl ::core::convert::From<InvalidReactor> for LimitOrderReactorErrors {
        fn from(value: InvalidReactor) -> Self {
            Self::InvalidReactor(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for LimitOrderReactorErrors {
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
    #[ethevent(name = "Fill", abi = "Fill(bytes32,address,address,uint256)")]
    pub struct FillFilter {
        #[ethevent(indexed)]
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub filler: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub swapper: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
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
        name = "ProtocolFeeControllerSet",
        abi = "ProtocolFeeControllerSet(address,address)"
    )]
    pub struct ProtocolFeeControllerSetFilter {
        pub old_fee_controller: ::ethers::core::types::Address,
        pub new_fee_controller: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LimitOrderReactorEvents {
        FillFilter(FillFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProtocolFeeControllerSetFilter(ProtocolFeeControllerSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for LimitOrderReactorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FillFilter::decode_log(log) {
                return Ok(LimitOrderReactorEvents::FillFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LimitOrderReactorEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProtocolFeeControllerSetFilter::decode_log(log) {
                return Ok(LimitOrderReactorEvents::ProtocolFeeControllerSetFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LimitOrderReactorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FillFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeControllerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FillFilter> for LimitOrderReactorEvents {
        fn from(value: FillFilter) -> Self {
            Self::FillFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LimitOrderReactorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeControllerSetFilter> for LimitOrderReactorEvents {
        fn from(value: ProtocolFeeControllerSetFilter) -> Self {
            Self::ProtocolFeeControllerSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `DIRECT_FILL` function with signature `DIRECT_FILL()` and selector `0xfccbcaaf`
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
    #[ethcall(name = "DIRECT_FILL", abi = "DIRECT_FILL()")]
    pub struct DirectFillCall;
    ///Container type for all input parameters for the `execute` function with signature `execute((bytes,bytes),address,bytes)` and selector `0x05afc977`
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
    #[ethcall(name = "execute", abi = "execute((bytes,bytes),address,bytes)")]
    pub struct ExecuteCall {
        pub order: SignedOrder,
        pub fill_contract: ::ethers::core::types::Address,
        pub fill_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch((bytes,bytes)[],address,bytes)` and selector `0x6f1d5f51`
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
        name = "executeBatch",
        abi = "executeBatch((bytes,bytes)[],address,bytes)"
    )]
    pub struct ExecuteBatchCall {
        pub orders: ::std::vec::Vec<SignedOrder>,
        pub fill_contract: ::ethers::core::types::Address,
        pub fill_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `feeController` function with signature `feeController()` and selector `0x6999b377`
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
    #[ethcall(name = "feeController", abi = "feeController()")]
    pub struct FeeControllerCall;
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
    ///Container type for all input parameters for the `permit2` function with signature `permit2()` and selector `0x12261ee7`
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
    #[ethcall(name = "permit2", abi = "permit2()")]
    pub struct Permit2Call;
    ///Container type for all input parameters for the `setProtocolFeeController` function with signature `setProtocolFeeController(address)` and selector `0x2d771389`
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
        name = "setProtocolFeeController",
        abi = "setProtocolFeeController(address)"
    )]
    pub struct SetProtocolFeeControllerCall {
        pub new_fee_controller: ::ethers::core::types::Address,
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LimitOrderReactorCalls {
        DirectFill(DirectFillCall),
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        FeeController(FeeControllerCall),
        Owner(OwnerCall),
        Permit2(Permit2Call),
        SetProtocolFeeController(SetProtocolFeeControllerCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for LimitOrderReactorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DirectFillCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DirectFill(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteBatchCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteBatch(decoded));
            }
            if let Ok(decoded) = <FeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeController(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <Permit2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit2(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProtocolFeeController(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LimitOrderReactorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DirectFill(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeController(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProtocolFeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LimitOrderReactorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DirectFill(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DirectFillCall> for LimitOrderReactorCalls {
        fn from(value: DirectFillCall) -> Self {
            Self::DirectFill(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for LimitOrderReactorCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for LimitOrderReactorCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<FeeControllerCall> for LimitOrderReactorCalls {
        fn from(value: FeeControllerCall) -> Self {
            Self::FeeController(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LimitOrderReactorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<Permit2Call> for LimitOrderReactorCalls {
        fn from(value: Permit2Call) -> Self {
            Self::Permit2(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeControllerCall> for LimitOrderReactorCalls {
        fn from(value: SetProtocolFeeControllerCall) -> Self {
            Self::SetProtocolFeeController(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LimitOrderReactorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `DIRECT_FILL` function with signature `DIRECT_FILL()` and selector `0xfccbcaaf`
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
    pub struct DirectFillReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeController` function with signature `feeController()` and selector `0x6999b377`
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
    pub struct FeeControllerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `permit2` function with signature `permit2()` and selector `0x12261ee7`
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
    pub struct Permit2Return(pub ::ethers::core::types::Address);
}
