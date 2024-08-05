pub use priority_order_reactor::*;
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
pub mod priority_order_reactor {
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
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeBatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeBatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orders"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeBatchWithCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeBatchWithCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("orders"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbackData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeWithCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "executeWithCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct SignedOrder"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callbackData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeController"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IProtocolFeeController",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("permit2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProtocolFeeController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setProtocolFeeController",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_newFeeController"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Fill"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Fill"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("orderHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolFeeControllerSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProtocolFeeControllerSet",
                            ),
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
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DuplicateFeeOutput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DuplicateFeeOutput"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("duplicateToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeTooLarge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InputAndOutputFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InputAndOutputFees"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InputOutputScaling"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InputOutputScaling"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCosignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidCosignature"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDeadline"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidDeadline"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeeToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidFeeToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidGasPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidGasPrice"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidReactor"),
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
                (
                    ::std::borrow::ToOwned::to_owned("OrderAlreadyFilled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OrderAlreadyFilled"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OrderNotFillable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OrderNotFillable"),
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
    pub static PRIORITYORDERREACTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\x004p8\x03\x80b\x004p\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xB8V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x84\x92\x84\x92\x83\x92\x83\x92\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3PP`\x01`\x02UP`\x01`\x01`\xA0\x1B\x03\x16`\x80RPb\0\0\xF7\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xB5W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xCCW`\0\x80\xFD[\x82Qb\0\0\xD9\x81b\0\0\x9FV[` \x84\x01Q\x90\x92Pb\0\0\xEC\x81b\0\0\x9FV[\x80\x91PP\x92P\x92\x90PV[`\x80Qa3Pb\0\x01 `\09`\0\x81\x81`\xE0\x01R\x81\x81a\ra\x01Ra\x19\xEB\x01Ra3P`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x9AW`\x005`\xE0\x1C\x80c-w\x13\x89\x11a\0iW\x80ci\x99\xB3w\x11a\0NW\x80ci\x99\xB3w\x14a\x01qW\x80c\x8D\xA5\xCB[\x14a\x01\x9EW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xCBW`\0\x80\xFD[\x80c-w\x13\x89\x14a\x01>W\x80c?b\x19.\x14a\x01^W`\0\x80\xFD[\x80c\r3X\x84\x14a\0\xA6W\x80c\rz\x16\xC3\x14a\0\xBBW\x80c\x12&\x1E\xE7\x14a\0\xCEW\x80c\x13\xFBr\xC7\x14a\x01+W`\0\x80\xFD[6a\0\xA1W\0[`\0\x80\xFD[a\0\xB9a\0\xB46`\x04a$@V[a\x01\xEBV[\0[a\0\xB9a\0\xC96`\x04a$\xEEV[a\x03dV[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\x01\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB9a\x0196`\x04a%0V[a\x04\xC5V[4\x80\x15a\x01JW`\0\x80\xFD[Pa\0\xB9a\x01Y6`\x04a%\xC9V[a\x06\x83V[a\0\xB9a\x01l6`\x04a%\xEDV[a\x07\x8FV[4\x80\x15a\x01}W`\0\x80\xFD[P`\x01Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xAAW`\0\x80\xFD[P`\0Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xD7W`\0\x80\xFD[Pa\0\xB9a\x01\xE66`\x04a%\xC9V[a\x08\x94V[a\x01\xF3a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x02\nW\x90PP\x90Pa\x02\xB2\x84a\t\xF6V[\x81`\0\x81Q\x81\x10a\x02\xC5Wa\x02\xC5a&YV[` \x02` \x01\x01\x81\x90RPa\x02\xD9\x81a\x0B`V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x03\x19\x90\x84\x90\x87\x90\x87\x90`\x04\x01a(\\V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPa\x03T\x81a\x0B\xB1V[Pa\x03_`\x01`\x02UV[PPPV[a\x03la\t\x85V[\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x88Wa\x03\x88a&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04CW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x03\xA6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x04\xA2Wa\x04}\x85\x85\x83\x81\x81\x10a\x04fWa\x04fa&YV[\x90P` \x02\x81\x01\x90a\x04x\x91\x90a)\"V[a\t\xF6V[\x82\x82\x81Q\x81\x10a\x04\x8FWa\x04\x8Fa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04IV[Pa\x04\xAC\x81a\x0B`V[a\x04\xB5\x81a\x0B\xB1V[PPa\x04\xC1`\x01`\x02UV[PPV[a\x04\xCDa\t\x85V[\x82`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xE9Wa\x04\xE9a&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xA4W\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x05\x07W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x05\xECWa\x05\xC7\x87\x87\x83\x81\x81\x10a\x04fWa\x04fa&YV[\x82\x82\x81Q\x81\x10a\x05\xD9Wa\x05\xD9a&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xAAV[Pa\x05\xF6\x81a\x0B`V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x066\x90\x84\x90\x88\x90\x88\x90`\x04\x01a(\\V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06PW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06dW=`\0\x80>=`\0\xFD[PPPPa\x06q\x81a\x0B\xB1V[PPa\x06}`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x07\x97a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x07\xAEW\x90PP\x90Pa\x08V\x82a\t\xF6V[\x81`\0\x81Q\x81\x10a\x08iWa\x08ia&YV[` \x02` \x01\x01\x81\x90RPa\x08}\x81a\x0B`V[a\x08\x86\x81a\x0B\xB1V[Pa\x08\x91`\x01`\x02UV[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\t\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\0V[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\no\x83\x80a)`V[\x81\x01\x90a\n|\x91\x90a-,V[\x80Q` \x81\x01Q`@\x90\x91\x01Q\x91\x92Pa\n\x95\x91a\r\x04V[`\0a\n\xA0\x82a\x0E\x17V[\x90Pa\n\xAC\x81\x83a\x10eV[`\0a\n\xBB\x83``\x01Qa\x11\xC7V[\x90P`@Q\x80`\xA0\x01`@R\x80\x84`\0\x01Q\x81R` \x01a\n\xE9\x83\x86`\x80\x01Qa\x12\x1C\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\x0B\x05\x83\x86`\xA0\x01Qa\x13\x1B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01\x86\x80` \x01\x90a\x0B\x1A\x91\x90a)`V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01\x92\x90\x92RP\x93\x92PPPV[\x80Q`\0[\x81\x81\x10\x15a\x03_W`\0\x83\x82\x81Q\x81\x10a\x0B\x81Wa\x0B\x81a&YV[` \x02` \x01\x01Q\x90Pa\x0B\x94\x81a\x14\0V[a\x0B\x9E\x813a\x18\xF0V[a\x0B\xA8\x813a\x19\xE9V[P`\x01\x01a\x0BeV[\x80Q`\0[\x81\x81\x10\x15a\x0C\xF3W`\0\x83\x82\x81Q\x81\x10a\x0B\xD2Wa\x0B\xD2a&YV[` \x02` \x01\x01Q\x90P`\0\x81`@\x01QQ\x90P`\0[\x81\x81\x10\x15a\x0CSW`\0\x83`@\x01Q\x82\x81Q\x81\x10a\x0C\tWa\x0C\ta&YV[` \x02` \x01\x01Q\x90Pa\x0CJ\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D#\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x0B\xE9V[P\x81`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x85\x81Q\x81\x10a\x0C\x9CWa\x0C\x9Ca&YV[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x85`\0\x01Q`@\x01Q`@Qa\x0C\xE1\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x01a\x0B\xB6V[PG\x15a\x04\xC1Wa\x04\xC13Ga\x1DjV[`@Q\x7FO\xE0+D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\x08\x83\x90\x1C`$\x83\x01\x81\x90R\x91`\x01`\xFF\x85\x16\x1B\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cO\xE0+D\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a.-V[\x90P\x81\x81\x18\x80\x83\x16`\0\x03a\x0E\x0FW`@Q\x7F\xEE;=K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`@Q\x7FPriorityOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`.\x82\x01R\x7Faddress cosigner,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`=\x82\x01R\x7Fuint256 auctionStartBlock,\0\0\0\0\0\0`N\x82\x01R\x7Fuint256 baselinePriorityFeeWei,\0`h\x82\x01R\x7FPriorityInput input,\0\0\0\0\0\0\0\0\0\0\0\0`\x87\x82\x01R\x7FPriorityOutput[] outputs)\0\0\0\0\0\0\0`\x9B\x82\x01R`\0\x90`\xB4\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\xC0\x83\x01\x90\x91R`\x8D\x80\x83R\x90\x91\x90a23` \x83\x019`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a1\xBD`H\x919`@Q\x80`\x80\x01`@R\x80`[\x81R` \x01a2\xC0`[\x919`@Q` \x01a\x0F\xB1\x94\x93\x92\x91\x90a.FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x0F\xD4\x83`\0\x01Qa\x1E\x04V[\x83` \x01Q\x84`@\x01Q\x85``\x01Qa\x0F\xF0\x87`\x80\x01Qa\x1E\x9EV[a\x0F\xFD\x88`\xA0\x01Qa\x1F\x05V[`@\x80Q` \x81\x01\x98\x90\x98R\x87\x01\x95\x90\x95Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16``\x86\x01R`\x80\x85\x01\x91\x90\x91R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q``\x01QB\x11\x15a\x10\xA4W`@Q\x7Fv\x9D\x11\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Q` \x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\x10\xD1WP\x80C\x10[\x80\x15a\x10\xE1WP`\xC0\x82\x01QQ\x81\x11[\x15a\x11\x0BW` \x82\x01Qa\x11\x03\x90a\x10\xF9\x84\x86a\x1F\xA3V[\x84`\xE0\x01Qa \x1AV[P`\xC0\x81\x01QQ[\x80C\x10\x15a\x11EW`@Q\x7F\xC6\x03U \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x01Q`@\x01Q\x15a\x03_W`\0[\x82`\xA0\x01QQ\x81\x10\x15a\x06}W`\0\x83`\xA0\x01Q\x82\x81Q\x81\x10a\x11{Wa\x11{a&YV[` \x02` \x01\x01Q`@\x01Q\x11\x15a\x11\xBFW`@Q\x7F\xA6\xB8D\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x11VV[`\0H:\x10\x15a\x12\x03W`@Q\x7F\xF3\xEBD\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PH:\x03\x81\x81\x11\x15a\x12\x13W\x03\x90V[P`\0[\x91\x90PV[a\x12V`@Q\x80``\x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x83`@\x01Q\x83a\x12h\x91\x90a.\xCCV[\x90Pb\x98\x96\x80\x81\x10a\x12\xB6W`@Q\x80``\x01`@R\x80\x85`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01\x85` \x01Q\x81RP\x91PPa\x13\x15V[`@\x80Q``\x81\x01\x90\x91R\x84Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x82\x15a\x13\0W` \x86\x01Qa\x12\xFB\x90b\x98\x96\x80\x85\x81\x03\x90a!BV[a\x13\x06V[\x85` \x01Q[\x81R` \x86\x81\x01Q\x91\x01R\x91PP[\x92\x91PPV[\x81Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x139Wa\x139a&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xA2W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x13WW\x90P[P\x91P`\0[\x81\x81\x10\x15a\x13\xF8Wa\x13\xD3\x85\x82\x81Q\x81\x10a\x13\xC5Wa\x13\xC5a&YV[` \x02` \x01\x01Q\x85a!~V[\x83\x82\x81Q\x81\x10a\x13\xE5Wa\x13\xE5a&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13\xA8V[PP\x92\x91PPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x14 WPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x14w\x90\x85\x90`\x04\x01a.\xE3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x14\xDA\x91\x90\x81\x01\x90a.\xF6V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x14\xF2\x82\x84a/\xC6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\nWa\x15\na&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15sW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x15(W\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\xC4W\x85`@\x01Q\x81\x81Q\x81\x10a\x15\x97Wa\x15\x97a&YV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x15\xB1Wa\x15\xB1a&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x15yV[P`\0\x80`\0[\x84\x81\x10\x15a\x18\xDFW`\0\x87\x82\x81Q\x81\x10a\x15\xE7Wa\x15\xE7a&YV[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x16\xA5W\x88\x81\x81Q\x81\x10a\x16\x0EWa\x16\x0Ea&YV[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x16\x9DW\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[`\x01\x01a\x15\xF4V[P`\0\x80[\x88\x81\x10\x15a\x17fW`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x16\xCAWa\x16\xCAa&YV[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17]W\x85\x15a\x17GW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x17V\x90\x84a/\xC6V[\x92P`\x01\x96P[P`\x01\x01a\x16\xAAV[P\x81Q` \x8B\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x17\xDFW\x84\x15a\x17\xC6W`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x8B\x01Q\x01Qa\x17\xD8\x90\x82a/\xC6V[\x90P`\x01\x93P[\x80`\0\x03a\x184W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[a\x18B\x81`\x05a'\x10a!BV[\x82` \x01Q\x11\x15a\x18\xB5W\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x07\0V[\x81\x86\x84\x8A\x01\x81Q\x81\x10a\x18\xCAWa\x18\xCAa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x15\xCBV[PPP`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x19AW`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x04\xC1W\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x19\xBD\x90\x84\x90\x86\x90`\x04\x01a/\xD9V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xD5W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0E\x0FW=`\0\x80>=`\0\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x1A\xA9\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82R\x80\x87\x01Q\x81\x01Q\x90\x82\x01R\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a23`\x8D\x919`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a1\xBD`H\x919`@\x80Q\x7FPriorityOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`.\x82\x01R\x7Faddress cosigner,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`=\x82\x01R\x7Fuint256 auctionStartBlock,\0\0\0\0\0\0`N\x82\x01R\x7Fuint256 baselinePriorityFeeWei,\0`h\x82\x01R\x7FPriorityInput input,\0\0\0\0\0\0\0\0\0\0\0\0`\x87\x82\x01R\x7FPriorityOutput[] outputs)\0\0\0\0\0\0\0`\x9B\x82\x01R\x81Q`\x94\x81\x83\x03\x01\x81Ra\x014\x82\x01\x90\x92R`[`\xB4\x82\x01\x81\x81R\x91a2\xC0\x90`\xD4\x019`@Q\x80``\x01`@R\x80`.\x81R` \x01a2\x05`.\x919`@Q` \x01a\x1C\x88\x95\x94\x93\x92\x91\x90a0\x08V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x1C\xF5\x96\x95\x94\x93\x92`\x04\x01a0\xA3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x0FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0FW=`\0\x80>=`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1DHWa\x03_\x82\x82a\x1DjV[a\x03_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x84\x84a\"2V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1D\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1D\xC9V[``\x91P[PP\x90P\x80a\x03_W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a23`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\x10H\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a1\xBD`H\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q\x90Qa\x10H\x95\x01\x93\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F%Wa\x1F%a&*V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1FOW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1F\x94W`\0a\x1F\x82\x85\x83\x81Q\x81\x10a\x1FuWa\x1Fua&YV[` \x02` \x01\x01Qa#$V[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a\x1FUV[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x81F\x84`\xC0\x01Q`@Q` \x01a\x1F\xBF\x91Q\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1F\xFC\x93\x92\x91` \x01a1kV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a 1\x91\x90a1\x98V[\x91P\x91P`\0\x83`@\x81Q\x81\x10a JWa Ja&YV[\x01` \x90\x81\x01Q`@\x80Q`\0\x80\x82R\x93\x81\x01\x80\x83R\x89\x90R`\xF8\x92\x90\x92\x1C\x90\x82\x01\x81\x90R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x92P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a \xA7W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80a!\x02WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a!9W`@Q\x7F\xD7\x81[\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a!wW`\0\x80\xFD[P\x91\x02\x04\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`@\x80Q``\x81\x01\x82R\x84Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R\x90\x84\x01Q` \x82\x01\x90\x15a\"\x01Wa!\xFC\x85`@\x01Q\x85a!\xE0\x91\x90a.\xCCV[a!\xED\x90b\x98\x96\x80a/\xC6V[` \x87\x01Q\x90b\x98\x96\x80a#\x9BV[a\"\x07V[\x84` \x01Q[\x81R` \x01\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90P\x92\x91PPV[`\0`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a#\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[PPPPPV[`\0`@Q\x80`\x80\x01`@R\x80`[\x81R` \x01a2\xC0`[\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qa\x10H\x96\x91\x92\x91\x01\x94\x85Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01R`@\x85\x01\x92\x90\x92R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a#\xD0W`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0`@\x82\x84\x03\x12\x15a#\xF1W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a$\tW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a$9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a$UW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$mW`\0\x80\xFD[a$y\x87\x83\x88\x01a#\xDFV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a$\x8FW`\0\x80\xFD[Pa$\x9C\x86\x82\x87\x01a#\xF7V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a$\xBBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xD3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a$9W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a%\x01W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x18W`\0\x80\xFD[a%$\x85\x82\x86\x01a$\xA9V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a%FW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%^W`\0\x80\xFD[a%j\x88\x83\x89\x01a$\xA9V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a%\x83W`\0\x80\xFD[Pa%\x90\x87\x82\x88\x01a#\xF7V[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\x91W`\0\x80\xFD[\x805a\x12\x17\x81a%\x9CV[`\0` \x82\x84\x03\x12\x15a%\xDBW`\0\x80\xFD[\x815a%\xE6\x81a%\x9CV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\xFFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\x16W`\0\x80\xFD[a&\"\x84\x82\x85\x01a#\xDFV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a&\xA3W\x81\x81\x01Q\x83\x82\x01R` \x01a&\x8BV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\xC4\x81` \x86\x01` \x86\x01a&\x88V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a'VW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'\x0BV[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra'\xD5a\x01\xA0\x85\x01\x82a&\xACV[\x90P` \x83\x01Qa(\x13` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra(+\x82\x82a&\xF6V[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra(E\x82\x82a&\xACV[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[`\0`@\x82\x01`@\x83R\x80\x86Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x89\x01`\0[\x83\x81\x10\x15a(\xD1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x85Ra(\xBF\x86\x83Qa'aV[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a(\x85V[PP\x85\x84\x03\x81\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x96\x95PPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a)VW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a)\x95W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\xB0W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a$9W`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xE8Wa)\xE8a&*V[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xE8Wa)\xE8a&*V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xE8Wa)\xE8a&*V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*|Wa*|a&*V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a*\x95W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xAFWa*\xAFa&*V[a*\xE0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a*5V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a*\xF5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a+$W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a+HWa+Ha&*V[\x81`@R\x82\x93P\x845\x91Pa+\\\x82a%\x9CV[\x90\x82R` \x84\x015\x90a+n\x82a%\x9CV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa+\x98\x82a%\x9CV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a+\xB2W`\0\x80\xFD[Pa+\xBF\x85\x82\x86\x01a*\x84V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a+\xDEW`\0\x80\xFD[a+\xE6a)\xC5V[\x90P\x815a+\xF3\x81a%\x9CV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a,+Wa,+a&*V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a,FW`\0\x80\xFD[\x815` a,[a,V\x83a,\x11V[a*5V[\x82\x81R`\x07\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a,zW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a,\xDFW`\x80\x81\x89\x03\x12\x15a,\x97W`\0\x80\x81\xFD[a,\x9Fa)\xEEV[\x815a,\xAA\x81a%\x9CV[\x81R\x81\x85\x015\x85\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015a,\xCD\x81a%\x9CV[\x90\x82\x01R\x83R\x91\x83\x01\x91`\x80\x01a,~V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a,\xFCW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a-\x1FWa-\x1Fa&*V[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a->W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a-VW`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a-kW`\0\x80\xFD[a-sa*\x11V[\x825\x82\x81\x11\x15a-\x82W`\0\x80\xFD[a-\x8E\x87\x82\x86\x01a+\x12V[\x82RPa-\x9D` \x84\x01a%\xBEV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01Ra-\xC3\x86`\x80\x85\x01a+\xCCV[`\x80\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a-\xDAW`\0\x80\xFD[a-\xE6\x87\x82\x86\x01a,5V[`\xA0\x83\x01RPa-\xFA\x86a\x01\0\x85\x01a,\xEAV[`\xC0\x82\x01Ra\x01 \x83\x015\x82\x81\x11\x15a.\x12W`\0\x80\xFD[a.\x1E\x87\x82\x86\x01a*\x84V[`\xE0\x83\x01RP\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a.?W`\0\x80\xFD[PQ\x91\x90PV[`\0\x85Qa.X\x81\x84` \x8A\x01a&\x88V[\x85Q\x90\x83\x01\x90a.l\x81\x83` \x8A\x01a&\x88V[\x85Q\x91\x01\x90a.\x7F\x81\x83` \x89\x01a&\x88V[\x84Q\x91\x01\x90a.\x92\x81\x83` \x88\x01a&\x88V[\x01\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x13\x15Wa\x13\x15a.\x9DV[` \x81R`\0a%\xE6` \x83\x01\x84a'aV[`\0` \x80\x83\x85\x03\x12\x15a/\tW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/ W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a/1W`\0\x80\xFD[\x80Qa/?a,V\x82a,\x11V[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a/^W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a/\xBAW\x80\x85\x8A\x03\x12\x15a/{W`\0\x80\x81\xFD[a/\x83a)\xC5V[\x85Qa/\x8E\x81a%\x9CV[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa/\xA7\x81a%\x9CV[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a/cV[P\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x13\x15Wa\x13\x15a.\x9DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a&\"`@\x83\x01\x84a'aV[\x7FPriorityOrder witness)\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x16\x87Qa0A\x81\x83\x86\x01` \x8C\x01a&\x88V[\x87Q\x90\x84\x01\x90a0W\x81\x84\x84\x01` \x8C\x01a&\x88V[\x87Q\x91\x01\x90a0l\x81\x84\x84\x01` \x8B\x01a&\x88V[\x86Q\x91\x01\x90a0\x81\x81\x84\x84\x01` \x8A\x01a&\x88V[\x85Q\x91\x01\x90a0\x96\x81\x84\x84\x01` \x89\x01a&\x88V[\x01\x01\x97\x96PPPPPPPV[`\0a\x01@a0\xD3\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra1\x14`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra1I\x81\x84\x01\x86a&\xACV[\x90P\x82\x81\x03a\x01 \x84\x01Ra1^\x81\x85a&\xACV[\x99\x98PPPPPPPPPV[\x83\x81R\x82` \x82\x01R`\0\x82Qa1\x89\x81`@\x85\x01` \x87\x01a&\x88V[\x91\x90\x91\x01`@\x01\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xABW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV\xFEPriorityInput(address token,uint256 amount,uint256 mpsPerPriorityFeeWei)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)PriorityOutput(address token,uint256 amount,uint256 mpsPerPriorityFeeWei,address recipient)\xA2dipfsX\"\x12 5>J\x9C\x8A\xF3,\xE6{]\xE3:\xAE\xCB_\xAE\x19\x8FF\x18\xEDv\xC7\xF0\xECf\xD0\x8D\xF4@&[dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static PRIORITYORDERREACTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x9AW`\x005`\xE0\x1C\x80c-w\x13\x89\x11a\0iW\x80ci\x99\xB3w\x11a\0NW\x80ci\x99\xB3w\x14a\x01qW\x80c\x8D\xA5\xCB[\x14a\x01\x9EW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xCBW`\0\x80\xFD[\x80c-w\x13\x89\x14a\x01>W\x80c?b\x19.\x14a\x01^W`\0\x80\xFD[\x80c\r3X\x84\x14a\0\xA6W\x80c\rz\x16\xC3\x14a\0\xBBW\x80c\x12&\x1E\xE7\x14a\0\xCEW\x80c\x13\xFBr\xC7\x14a\x01+W`\0\x80\xFD[6a\0\xA1W\0[`\0\x80\xFD[a\0\xB9a\0\xB46`\x04a$@V[a\x01\xEBV[\0[a\0\xB9a\0\xC96`\x04a$\xEEV[a\x03dV[4\x80\x15a\0\xDAW`\0\x80\xFD[Pa\x01\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB9a\x0196`\x04a%0V[a\x04\xC5V[4\x80\x15a\x01JW`\0\x80\xFD[Pa\0\xB9a\x01Y6`\x04a%\xC9V[a\x06\x83V[a\0\xB9a\x01l6`\x04a%\xEDV[a\x07\x8FV[4\x80\x15a\x01}W`\0\x80\xFD[P`\x01Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xAAW`\0\x80\xFD[P`\0Ta\x01\x02\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xD7W`\0\x80\xFD[Pa\0\xB9a\x01\xE66`\x04a%\xC9V[a\x08\x94V[a\x01\xF3a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x02\nW\x90PP\x90Pa\x02\xB2\x84a\t\xF6V[\x81`\0\x81Q\x81\x10a\x02\xC5Wa\x02\xC5a&YV[` \x02` \x01\x01\x81\x90RPa\x02\xD9\x81a\x0B`V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x03\x19\x90\x84\x90\x87\x90\x87\x90`\x04\x01a(\\V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x033W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03GW=`\0\x80>=`\0\xFD[PPPPa\x03T\x81a\x0B\xB1V[Pa\x03_`\x01`\x02UV[PPPV[a\x03la\t\x85V[\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x88Wa\x03\x88a&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04CW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x03\xA6W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x04\xA2Wa\x04}\x85\x85\x83\x81\x81\x10a\x04fWa\x04fa&YV[\x90P` \x02\x81\x01\x90a\x04x\x91\x90a)\"V[a\t\xF6V[\x82\x82\x81Q\x81\x10a\x04\x8FWa\x04\x8Fa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04IV[Pa\x04\xAC\x81a\x0B`V[a\x04\xB5\x81a\x0B\xB1V[PPa\x04\xC1`\x01`\x02UV[PPV[a\x04\xCDa\t\x85V[\x82`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xE9Wa\x04\xE9a&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xA4W\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x05\x07W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x05\xECWa\x05\xC7\x87\x87\x83\x81\x81\x10a\x04fWa\x04fa&YV[\x82\x82\x81Q\x81\x10a\x05\xD9Wa\x05\xD9a&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xAAV[Pa\x05\xF6\x81a\x0B`V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x066\x90\x84\x90\x88\x90\x88\x90`\x04\x01a(\\V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06PW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06dW=`\0\x80>=`\0\xFD[PPPPa\x06q\x81a\x0B\xB1V[PPa\x06}`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07\tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x07\x97a\t\x85V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x07\xAEW\x90PP\x90Pa\x08V\x82a\t\xF6V[\x81`\0\x81Q\x81\x10a\x08iWa\x08ia&YV[` \x02` \x01\x01\x81\x90RPa\x08}\x81a\x0B`V[a\x08\x86\x81a\x0B\xB1V[Pa\x08\x91`\x01`\x02UV[PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\x15W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\t\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\0V[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\no\x83\x80a)`V[\x81\x01\x90a\n|\x91\x90a-,V[\x80Q` \x81\x01Q`@\x90\x91\x01Q\x91\x92Pa\n\x95\x91a\r\x04V[`\0a\n\xA0\x82a\x0E\x17V[\x90Pa\n\xAC\x81\x83a\x10eV[`\0a\n\xBB\x83``\x01Qa\x11\xC7V[\x90P`@Q\x80`\xA0\x01`@R\x80\x84`\0\x01Q\x81R` \x01a\n\xE9\x83\x86`\x80\x01Qa\x12\x1C\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\x0B\x05\x83\x86`\xA0\x01Qa\x13\x1B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01\x86\x80` \x01\x90a\x0B\x1A\x91\x90a)`V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01\x92\x90\x92RP\x93\x92PPPV[\x80Q`\0[\x81\x81\x10\x15a\x03_W`\0\x83\x82\x81Q\x81\x10a\x0B\x81Wa\x0B\x81a&YV[` \x02` \x01\x01Q\x90Pa\x0B\x94\x81a\x14\0V[a\x0B\x9E\x813a\x18\xF0V[a\x0B\xA8\x813a\x19\xE9V[P`\x01\x01a\x0BeV[\x80Q`\0[\x81\x81\x10\x15a\x0C\xF3W`\0\x83\x82\x81Q\x81\x10a\x0B\xD2Wa\x0B\xD2a&YV[` \x02` \x01\x01Q\x90P`\0\x81`@\x01QQ\x90P`\0[\x81\x81\x10\x15a\x0CSW`\0\x83`@\x01Q\x82\x81Q\x81\x10a\x0C\tWa\x0C\ta&YV[` \x02` \x01\x01Q\x90Pa\x0CJ\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1D#\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x0B\xE9V[P\x81`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x85\x81Q\x81\x10a\x0C\x9CWa\x0C\x9Ca&YV[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x85`\0\x01Q`@\x01Q`@Qa\x0C\xE1\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x01a\x0B\xB6V[PG\x15a\x04\xC1Wa\x04\xC13Ga\x1DjV[`@Q\x7FO\xE0+D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\x08\x83\x90\x1C`$\x83\x01\x81\x90R\x91`\x01`\xFF\x85\x16\x1B\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cO\xE0+D\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCE\x91\x90a.-V[\x90P\x81\x81\x18\x80\x83\x16`\0\x03a\x0E\x0FW`@Q\x7F\xEE;=K\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`@Q\x7FPriorityOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`.\x82\x01R\x7Faddress cosigner,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`=\x82\x01R\x7Fuint256 auctionStartBlock,\0\0\0\0\0\0`N\x82\x01R\x7Fuint256 baselinePriorityFeeWei,\0`h\x82\x01R\x7FPriorityInput input,\0\0\0\0\0\0\0\0\0\0\0\0`\x87\x82\x01R\x7FPriorityOutput[] outputs)\0\0\0\0\0\0\0`\x9B\x82\x01R`\0\x90`\xB4\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\xC0\x83\x01\x90\x91R`\x8D\x80\x83R\x90\x91\x90a23` \x83\x019`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a1\xBD`H\x919`@Q\x80`\x80\x01`@R\x80`[\x81R` \x01a2\xC0`[\x919`@Q` \x01a\x0F\xB1\x94\x93\x92\x91\x90a.FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x0F\xD4\x83`\0\x01Qa\x1E\x04V[\x83` \x01Q\x84`@\x01Q\x85``\x01Qa\x0F\xF0\x87`\x80\x01Qa\x1E\x9EV[a\x0F\xFD\x88`\xA0\x01Qa\x1F\x05V[`@\x80Q` \x81\x01\x98\x90\x98R\x87\x01\x95\x90\x95Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x93\x16``\x86\x01R`\x80\x85\x01\x91\x90\x91R`\xA0\x84\x01R`\xC0\x83\x01R`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q``\x01QB\x11\x15a\x10\xA4W`@Q\x7Fv\x9D\x11\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Q` \x82\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\x10\xD1WP\x80C\x10[\x80\x15a\x10\xE1WP`\xC0\x82\x01QQ\x81\x11[\x15a\x11\x0BW` \x82\x01Qa\x11\x03\x90a\x10\xF9\x84\x86a\x1F\xA3V[\x84`\xE0\x01Qa \x1AV[P`\xC0\x81\x01QQ[\x80C\x10\x15a\x11EW`@Q\x7F\xC6\x03U \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x82\x01Q`@\x01Q\x15a\x03_W`\0[\x82`\xA0\x01QQ\x81\x10\x15a\x06}W`\0\x83`\xA0\x01Q\x82\x81Q\x81\x10a\x11{Wa\x11{a&YV[` \x02` \x01\x01Q`@\x01Q\x11\x15a\x11\xBFW`@Q\x7F\xA6\xB8D\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x11VV[`\0H:\x10\x15a\x12\x03W`@Q\x7F\xF3\xEBD\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PH:\x03\x81\x81\x11\x15a\x12\x13W\x03\x90V[P`\0[\x91\x90PV[a\x12V`@Q\x80``\x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x83`@\x01Q\x83a\x12h\x91\x90a.\xCCV[\x90Pb\x98\x96\x80\x81\x10a\x12\xB6W`@Q\x80``\x01`@R\x80\x85`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01\x85` \x01Q\x81RP\x91PPa\x13\x15V[`@\x80Q``\x81\x01\x90\x91R\x84Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x82\x15a\x13\0W` \x86\x01Qa\x12\xFB\x90b\x98\x96\x80\x85\x81\x03\x90a!BV[a\x13\x06V[\x85` \x01Q[\x81R` \x86\x81\x01Q\x91\x01R\x91PP[\x92\x91PPV[\x81Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x139Wa\x139a&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xA2W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x13WW\x90P[P\x91P`\0[\x81\x81\x10\x15a\x13\xF8Wa\x13\xD3\x85\x82\x81Q\x81\x10a\x13\xC5Wa\x13\xC5a&YV[` \x02` \x01\x01Q\x85a!~V[\x83\x82\x81Q\x81\x10a\x13\xE5Wa\x13\xE5a&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x13\xA8V[PP\x92\x91PPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x14 WPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x14w\x90\x85\x90`\x04\x01a.\xE3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x14\xDA\x91\x90\x81\x01\x90a.\xF6V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x14\xF2\x82\x84a/\xC6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\nWa\x15\na&*V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15sW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x15(W\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\xC4W\x85`@\x01Q\x81\x81Q\x81\x10a\x15\x97Wa\x15\x97a&YV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x15\xB1Wa\x15\xB1a&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x15yV[P`\0\x80`\0[\x84\x81\x10\x15a\x18\xDFW`\0\x87\x82\x81Q\x81\x10a\x15\xE7Wa\x15\xE7a&YV[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x16\xA5W\x88\x81\x81Q\x81\x10a\x16\x0EWa\x16\x0Ea&YV[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x16\x9DW\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[`\x01\x01a\x15\xF4V[P`\0\x80[\x88\x81\x10\x15a\x17fW`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x16\xCAWa\x16\xCAa&YV[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x17]W\x85\x15a\x17GW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x17V\x90\x84a/\xC6V[\x92P`\x01\x96P[P`\x01\x01a\x16\xAAV[P\x81Q` \x8B\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x17\xDFW\x84\x15a\x17\xC6W`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x8B\x01Q\x01Qa\x17\xD8\x90\x82a/\xC6V[\x90P`\x01\x93P[\x80`\0\x03a\x184W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07\0V[a\x18B\x81`\x05a'\x10a!BV[\x82` \x01Q\x11\x15a\x18\xB5W\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x07\0V[\x81\x86\x84\x8A\x01\x81Q\x81\x10a\x18\xCAWa\x18\xCAa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x15\xCBV[PPP`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x19AW`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x04\xC1W\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x19\xBD\x90\x84\x90\x86\x90`\x04\x01a/\xD9V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19\xD5W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x0E\x0FW=`\0\x80>=`\0\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x1A\xA9\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82R\x80\x87\x01Q\x81\x01Q\x90\x82\x01R\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a23`\x8D\x919`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a1\xBD`H\x919`@\x80Q\x7FPriorityOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`.\x82\x01R\x7Faddress cosigner,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`=\x82\x01R\x7Fuint256 auctionStartBlock,\0\0\0\0\0\0`N\x82\x01R\x7Fuint256 baselinePriorityFeeWei,\0`h\x82\x01R\x7FPriorityInput input,\0\0\0\0\0\0\0\0\0\0\0\0`\x87\x82\x01R\x7FPriorityOutput[] outputs)\0\0\0\0\0\0\0`\x9B\x82\x01R\x81Q`\x94\x81\x83\x03\x01\x81Ra\x014\x82\x01\x90\x92R`[`\xB4\x82\x01\x81\x81R\x91a2\xC0\x90`\xD4\x019`@Q\x80``\x01`@R\x80`.\x81R` \x01a2\x05`.\x919`@Q` \x01a\x1C\x88\x95\x94\x93\x92\x91\x90a0\x08V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x1C\xF5\x96\x95\x94\x93\x92`\x04\x01a0\xA3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x0FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0FW=`\0\x80>=`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1DHWa\x03_\x82\x82a\x1DjV[a\x03_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x84\x84a\"2V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x1D\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1D\xC9V[``\x91P[PP\x90P\x80a\x03_W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a23`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\x10H\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a1\xBD`H\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q\x90Qa\x10H\x95\x01\x93\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16` \x84\x01R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1F%Wa\x1F%a&*V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1FOW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1F\x94W`\0a\x1F\x82\x85\x83\x81Q\x81\x10a\x1FuWa\x1Fua&YV[` \x02` \x01\x01Qa#$V[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a\x1FUV[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x81F\x84`\xC0\x01Q`@Q` \x01a\x1F\xBF\x91Q\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1F\xFC\x93\x92\x91` \x01a1kV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a 1\x91\x90a1\x98V[\x91P\x91P`\0\x83`@\x81Q\x81\x10a JWa Ja&YV[\x01` \x90\x81\x01Q`@\x80Q`\0\x80\x82R\x93\x81\x01\x80\x83R\x89\x90R`\xF8\x92\x90\x92\x1C\x90\x82\x01\x81\x90R``\x82\x01\x86\x90R`\x80\x82\x01\x85\x90R\x92P`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a \xA7W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15\x80a!\x02WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x15[\x15a!9W`@Q\x7F\xD7\x81[\xE1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a!wW`\0\x80\xFD[P\x91\x02\x04\x90V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`@\x80Q``\x81\x01\x82R\x84Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R\x90\x84\x01Q` \x82\x01\x90\x15a\"\x01Wa!\xFC\x85`@\x01Q\x85a!\xE0\x91\x90a.\xCCV[a!\xED\x90b\x98\x96\x80a/\xC6V[` \x87\x01Q\x90b\x98\x96\x80a#\x9BV[a\"\x07V[\x84` \x01Q[\x81R` \x01\x84``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90P\x92\x91PPV[`\0`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a#\x1DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\0V[PPPPPV[`\0`@Q\x80`\x80\x01`@R\x80`[\x81R` \x01a2\xC0`[\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qa\x10H\x96\x91\x92\x91\x01\x94\x85Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01R`@\x85\x01\x92\x90\x92R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a#\xD0W`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0`@\x82\x84\x03\x12\x15a#\xF1W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a$\tW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a$9W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a$UW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$mW`\0\x80\xFD[a$y\x87\x83\x88\x01a#\xDFV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a$\x8FW`\0\x80\xFD[Pa$\x9C\x86\x82\x87\x01a#\xF7V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a$\xBBW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xD3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a$9W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a%\x01W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\x18W`\0\x80\xFD[a%$\x85\x82\x86\x01a$\xA9V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a%FW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%^W`\0\x80\xFD[a%j\x88\x83\x89\x01a$\xA9V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a%\x83W`\0\x80\xFD[Pa%\x90\x87\x82\x88\x01a#\xF7V[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\x91W`\0\x80\xFD[\x805a\x12\x17\x81a%\x9CV[`\0` \x82\x84\x03\x12\x15a%\xDBW`\0\x80\xFD[\x815a%\xE6\x81a%\x9CV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\xFFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\x16W`\0\x80\xFD[a&\"\x84\x82\x85\x01a#\xDFV[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a&\xA3W\x81\x81\x01Q\x83\x82\x01R` \x01a&\x8BV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra&\xC4\x81` \x86\x01` \x86\x01a&\x88V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a'VW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'\x0BV[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra'\xD5a\x01\xA0\x85\x01\x82a&\xACV[\x90P` \x83\x01Qa(\x13` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra(+\x82\x82a&\xF6V[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra(E\x82\x82a&\xACV[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[`\0`@\x82\x01`@\x83R\x80\x86Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x89\x01`\0[\x83\x81\x10\x15a(\xD1W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x85Ra(\xBF\x86\x83Qa'aV[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a(\x85V[PP\x85\x84\x03\x81\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x96\x95PPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a)VW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a)\x95W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\xB0W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a$9W`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xE8Wa)\xE8a&*V[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xE8Wa)\xE8a&*V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\xE8Wa)\xE8a&*V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*|Wa*|a&*V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a*\x95W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xAFWa*\xAFa&*V[a*\xE0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a*5V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a*\xF5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a+$W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a+HWa+Ha&*V[\x81`@R\x82\x93P\x845\x91Pa+\\\x82a%\x9CV[\x90\x82R` \x84\x015\x90a+n\x82a%\x9CV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa+\x98\x82a%\x9CV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a+\xB2W`\0\x80\xFD[Pa+\xBF\x85\x82\x86\x01a*\x84V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a+\xDEW`\0\x80\xFD[a+\xE6a)\xC5V[\x90P\x815a+\xF3\x81a%\x9CV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a,+Wa,+a&*V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a,FW`\0\x80\xFD[\x815` a,[a,V\x83a,\x11V[a*5V[\x82\x81R`\x07\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a,zW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a,\xDFW`\x80\x81\x89\x03\x12\x15a,\x97W`\0\x80\x81\xFD[a,\x9Fa)\xEEV[\x815a,\xAA\x81a%\x9CV[\x81R\x81\x85\x015\x85\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015a,\xCD\x81a%\x9CV[\x90\x82\x01R\x83R\x91\x83\x01\x91`\x80\x01a,~V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a,\xFCW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a-\x1FWa-\x1Fa&*V[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a->W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a-VW`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a-kW`\0\x80\xFD[a-sa*\x11V[\x825\x82\x81\x11\x15a-\x82W`\0\x80\xFD[a-\x8E\x87\x82\x86\x01a+\x12V[\x82RPa-\x9D` \x84\x01a%\xBEV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01Ra-\xC3\x86`\x80\x85\x01a+\xCCV[`\x80\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a-\xDAW`\0\x80\xFD[a-\xE6\x87\x82\x86\x01a,5V[`\xA0\x83\x01RPa-\xFA\x86a\x01\0\x85\x01a,\xEAV[`\xC0\x82\x01Ra\x01 \x83\x015\x82\x81\x11\x15a.\x12W`\0\x80\xFD[a.\x1E\x87\x82\x86\x01a*\x84V[`\xE0\x83\x01RP\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a.?W`\0\x80\xFD[PQ\x91\x90PV[`\0\x85Qa.X\x81\x84` \x8A\x01a&\x88V[\x85Q\x90\x83\x01\x90a.l\x81\x83` \x8A\x01a&\x88V[\x85Q\x91\x01\x90a.\x7F\x81\x83` \x89\x01a&\x88V[\x84Q\x91\x01\x90a.\x92\x81\x83` \x88\x01a&\x88V[\x01\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x13\x15Wa\x13\x15a.\x9DV[` \x81R`\0a%\xE6` \x83\x01\x84a'aV[`\0` \x80\x83\x85\x03\x12\x15a/\tW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/ W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a/1W`\0\x80\xFD[\x80Qa/?a,V\x82a,\x11V[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a/^W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a/\xBAW\x80\x85\x8A\x03\x12\x15a/{W`\0\x80\x81\xFD[a/\x83a)\xC5V[\x85Qa/\x8E\x81a%\x9CV[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa/\xA7\x81a%\x9CV[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a/cV[P\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x13\x15Wa\x13\x15a.\x9DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a&\"`@\x83\x01\x84a'aV[\x7FPriorityOrder witness)\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x16\x87Qa0A\x81\x83\x86\x01` \x8C\x01a&\x88V[\x87Q\x90\x84\x01\x90a0W\x81\x84\x84\x01` \x8C\x01a&\x88V[\x87Q\x91\x01\x90a0l\x81\x84\x84\x01` \x8B\x01a&\x88V[\x86Q\x91\x01\x90a0\x81\x81\x84\x84\x01` \x8A\x01a&\x88V[\x85Q\x91\x01\x90a0\x96\x81\x84\x84\x01` \x89\x01a&\x88V[\x01\x01\x97\x96PPPPPPPV[`\0a\x01@a0\xD3\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra1\x14`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra1I\x81\x84\x01\x86a&\xACV[\x90P\x82\x81\x03a\x01 \x84\x01Ra1^\x81\x85a&\xACV[\x99\x98PPPPPPPPPV[\x83\x81R\x82` \x82\x01R`\0\x82Qa1\x89\x81`@\x85\x01` \x87\x01a&\x88V[\x91\x90\x91\x01`@\x01\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xABW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV\xFEPriorityInput(address token,uint256 amount,uint256 mpsPerPriorityFeeWei)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)PriorityOutput(address token,uint256 amount,uint256 mpsPerPriorityFeeWei,address recipient)\xA2dipfsX\"\x12 5>J\x9C\x8A\xF3,\xE6{]\xE3:\xAE\xCB_\xAE\x19\x8FF\x18\xEDv\xC7\xF0\xECf\xD0\x8D\xF4@&[dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static PRIORITYORDERREACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PriorityOrderReactor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PriorityOrderReactor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PriorityOrderReactor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PriorityOrderReactor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PriorityOrderReactor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PriorityOrderReactor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PriorityOrderReactor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PRIORITYORDERREACTOR_ABI.clone(),
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
                PRIORITYORDERREACTOR_ABI.clone(),
                PRIORITYORDERREACTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `execute` (0x3f62192e) function
        pub fn execute(
            &self,
            order: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 98, 25, 46], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatch` (0x0d7a16c3) function
        pub fn execute_batch(
            &self,
            orders: ::std::vec::Vec<SignedOrder>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 122, 22, 195], orders)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeBatchWithCallback` (0x13fb72c7) function
        pub fn execute_batch_with_callback(
            &self,
            orders: ::std::vec::Vec<SignedOrder>,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 251, 114, 199], (orders, callback_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithCallback` (0x0d335884) function
        pub fn execute_with_callback(
            &self,
            order: SignedOrder,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 51, 88, 132], (order, callback_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeController` (0x6999b377) function
        pub fn fee_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([105, 153, 179, 119], ())
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
        ///Calls the contract's `permit2` (0x12261ee7) function
        pub fn permit_2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriorityOrderReactorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PriorityOrderReactor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DuplicateFeeOutput` with signature `DuplicateFeeOutput(address)` and selector `0xfff08303`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FeeTooLarge", abi = "FeeTooLarge(address,uint256,address)")]
    pub struct FeeTooLarge {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Custom Error type `InputAndOutputFees` with signature `InputAndOutputFees()` and selector `0xedc7e2e4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InputAndOutputFees", abi = "InputAndOutputFees()")]
    pub struct InputAndOutputFees;
    ///Custom Error type `InputOutputScaling` with signature `InputOutputScaling()` and selector `0xa6b844f5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InputOutputScaling", abi = "InputOutputScaling()")]
    pub struct InputOutputScaling;
    ///Custom Error type `InvalidCosignature` with signature `InvalidCosignature()` and selector `0xd7815be1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidCosignature", abi = "InvalidCosignature()")]
    pub struct InvalidCosignature;
    ///Custom Error type `InvalidDeadline` with signature `InvalidDeadline()` and selector `0x769d11e4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidDeadline", abi = "InvalidDeadline()")]
    pub struct InvalidDeadline;
    ///Custom Error type `InvalidFeeToken` with signature `InvalidFeeToken(address)` and selector `0xeddf07f5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidFeeToken", abi = "InvalidFeeToken(address)")]
    pub struct InvalidFeeToken {
        pub fee_token: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidGasPrice` with signature `InvalidGasPrice()` and selector `0xf3eb44e5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidGasPrice", abi = "InvalidGasPrice()")]
    pub struct InvalidGasPrice;
    ///Custom Error type `InvalidReactor` with signature `InvalidReactor()` and selector `0x4ddf4a64`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidReactor", abi = "InvalidReactor()")]
    pub struct InvalidReactor;
    ///Custom Error type `NativeTransferFailed` with signature `NativeTransferFailed()` and selector `0xf4b3b1bc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NativeTransferFailed", abi = "NativeTransferFailed()")]
    pub struct NativeTransferFailed;
    ///Custom Error type `OrderAlreadyFilled` with signature `OrderAlreadyFilled()` and selector `0xee3b3d4b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OrderAlreadyFilled", abi = "OrderAlreadyFilled()")]
    pub struct OrderAlreadyFilled;
    ///Custom Error type `OrderNotFillable` with signature `OrderNotFillable()` and selector `0xc6035520`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OrderNotFillable", abi = "OrderNotFillable()")]
    pub struct OrderNotFillable;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum PriorityOrderReactorErrors {
        DuplicateFeeOutput(DuplicateFeeOutput),
        FeeTooLarge(FeeTooLarge),
        InputAndOutputFees(InputAndOutputFees),
        InputOutputScaling(InputOutputScaling),
        InvalidCosignature(InvalidCosignature),
        InvalidDeadline(InvalidDeadline),
        InvalidFeeToken(InvalidFeeToken),
        InvalidGasPrice(InvalidGasPrice),
        InvalidReactor(InvalidReactor),
        NativeTransferFailed(NativeTransferFailed),
        OrderAlreadyFilled(OrderAlreadyFilled),
        OrderNotFillable(OrderNotFillable),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PriorityOrderReactorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DuplicateFeeOutput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DuplicateFeeOutput(decoded));
            }
            if let Ok(decoded) = <FeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeTooLarge(decoded));
            }
            if let Ok(decoded) = <InputAndOutputFees as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InputAndOutputFees(decoded));
            }
            if let Ok(decoded) = <InputOutputScaling as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InputOutputScaling(decoded));
            }
            if let Ok(decoded) = <InvalidCosignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCosignature(decoded));
            }
            if let Ok(decoded) = <InvalidDeadline as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidDeadline(decoded));
            }
            if let Ok(decoded) = <InvalidFeeToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFeeToken(decoded));
            }
            if let Ok(decoded) = <InvalidGasPrice as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidGasPrice(decoded));
            }
            if let Ok(decoded) = <InvalidReactor as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReactor(decoded));
            }
            if let Ok(decoded) = <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NativeTransferFailed(decoded));
            }
            if let Ok(decoded) = <OrderAlreadyFilled as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OrderAlreadyFilled(decoded));
            }
            if let Ok(decoded) = <OrderNotFillable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OrderNotFillable(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PriorityOrderReactorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DuplicateFeeOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InputAndOutputFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InputOutputScaling(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCosignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDeadline(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidGasPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderAlreadyFilled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderNotFillable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PriorityOrderReactorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DuplicateFeeOutput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FeeTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InputAndOutputFees as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InputOutputScaling as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCosignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidDeadline as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFeeToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidGasPrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidReactor as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NativeTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OrderAlreadyFilled as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OrderNotFillable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PriorityOrderReactorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DuplicateFeeOutput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::InputAndOutputFees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InputOutputScaling(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCosignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidDeadline(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidGasPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderAlreadyFilled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderNotFillable(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PriorityOrderReactorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DuplicateFeeOutput> for PriorityOrderReactorErrors {
        fn from(value: DuplicateFeeOutput) -> Self {
            Self::DuplicateFeeOutput(value)
        }
    }
    impl ::core::convert::From<FeeTooLarge> for PriorityOrderReactorErrors {
        fn from(value: FeeTooLarge) -> Self {
            Self::FeeTooLarge(value)
        }
    }
    impl ::core::convert::From<InputAndOutputFees> for PriorityOrderReactorErrors {
        fn from(value: InputAndOutputFees) -> Self {
            Self::InputAndOutputFees(value)
        }
    }
    impl ::core::convert::From<InputOutputScaling> for PriorityOrderReactorErrors {
        fn from(value: InputOutputScaling) -> Self {
            Self::InputOutputScaling(value)
        }
    }
    impl ::core::convert::From<InvalidCosignature> for PriorityOrderReactorErrors {
        fn from(value: InvalidCosignature) -> Self {
            Self::InvalidCosignature(value)
        }
    }
    impl ::core::convert::From<InvalidDeadline> for PriorityOrderReactorErrors {
        fn from(value: InvalidDeadline) -> Self {
            Self::InvalidDeadline(value)
        }
    }
    impl ::core::convert::From<InvalidFeeToken> for PriorityOrderReactorErrors {
        fn from(value: InvalidFeeToken) -> Self {
            Self::InvalidFeeToken(value)
        }
    }
    impl ::core::convert::From<InvalidGasPrice> for PriorityOrderReactorErrors {
        fn from(value: InvalidGasPrice) -> Self {
            Self::InvalidGasPrice(value)
        }
    }
    impl ::core::convert::From<InvalidReactor> for PriorityOrderReactorErrors {
        fn from(value: InvalidReactor) -> Self {
            Self::InvalidReactor(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for PriorityOrderReactorErrors {
        fn from(value: NativeTransferFailed) -> Self {
            Self::NativeTransferFailed(value)
        }
    }
    impl ::core::convert::From<OrderAlreadyFilled> for PriorityOrderReactorErrors {
        fn from(value: OrderAlreadyFilled) -> Self {
            Self::OrderAlreadyFilled(value)
        }
    }
    impl ::core::convert::From<OrderNotFillable> for PriorityOrderReactorErrors {
        fn from(value: OrderNotFillable) -> Self {
            Self::OrderNotFillable(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        serde::Serialize,
        serde::Deserialize,
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
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum PriorityOrderReactorEvents {
        FillFilter(FillFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProtocolFeeControllerSetFilter(ProtocolFeeControllerSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for PriorityOrderReactorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FillFilter::decode_log(log) {
                return Ok(PriorityOrderReactorEvents::FillFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    PriorityOrderReactorEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = ProtocolFeeControllerSetFilter::decode_log(log) {
                return Ok(
                    PriorityOrderReactorEvents::ProtocolFeeControllerSetFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PriorityOrderReactorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FillFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProtocolFeeControllerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FillFilter> for PriorityOrderReactorEvents {
        fn from(value: FillFilter) -> Self {
            Self::FillFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for PriorityOrderReactorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeControllerSetFilter>
    for PriorityOrderReactorEvents {
        fn from(value: ProtocolFeeControllerSetFilter) -> Self {
            Self::ProtocolFeeControllerSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `execute` function with signature `execute((bytes,bytes))` and selector `0x3f62192e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "execute", abi = "execute((bytes,bytes))")]
    pub struct ExecuteCall {
        pub order: SignedOrder,
    }
    ///Container type for all input parameters for the `executeBatch` function with signature `executeBatch((bytes,bytes)[])` and selector `0x0d7a16c3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "executeBatch", abi = "executeBatch((bytes,bytes)[])")]
    pub struct ExecuteBatchCall {
        pub orders: ::std::vec::Vec<SignedOrder>,
    }
    ///Container type for all input parameters for the `executeBatchWithCallback` function with signature `executeBatchWithCallback((bytes,bytes)[],bytes)` and selector `0x13fb72c7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeBatchWithCallback",
        abi = "executeBatchWithCallback((bytes,bytes)[],bytes)"
    )]
    pub struct ExecuteBatchWithCallbackCall {
        pub orders: ::std::vec::Vec<SignedOrder>,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeWithCallback` function with signature `executeWithCallback((bytes,bytes),bytes)` and selector `0x0d335884`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeWithCallback",
        abi = "executeWithCallback((bytes,bytes),bytes)"
    )]
    pub struct ExecuteWithCallbackCall {
        pub order: SignedOrder,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `feeController` function with signature `feeController()` and selector `0x6999b377`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feeController", abi = "feeController()")]
    pub struct FeeControllerCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `permit2` function with signature `permit2()` and selector `0x12261ee7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "permit2", abi = "permit2()")]
    pub struct Permit2Call;
    ///Container type for all input parameters for the `setProtocolFeeController` function with signature `setProtocolFeeController(address)` and selector `0x2d771389`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
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
        serde::Serialize,
        serde::Deserialize,
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
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum PriorityOrderReactorCalls {
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        ExecuteBatchWithCallback(ExecuteBatchWithCallbackCall),
        ExecuteWithCallback(ExecuteWithCallbackCall),
        FeeController(FeeControllerCall),
        Owner(OwnerCall),
        Permit2(Permit2Call),
        SetProtocolFeeController(SetProtocolFeeControllerCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for PriorityOrderReactorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteBatchCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBatch(decoded));
            }
            if let Ok(decoded) = <ExecuteBatchWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteBatchWithCallback(decoded));
            }
            if let Ok(decoded) = <ExecuteWithCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteWithCallback(decoded));
            }
            if let Ok(decoded) = <FeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeController(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <Permit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2(decoded));
            }
            if let Ok(decoded) = <SetProtocolFeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProtocolFeeController(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PriorityOrderReactorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteBatchWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProtocolFeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PriorityOrderReactorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatchWithCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteWithCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for PriorityOrderReactorCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for PriorityOrderReactorCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchWithCallbackCall>
    for PriorityOrderReactorCalls {
        fn from(value: ExecuteBatchWithCallbackCall) -> Self {
            Self::ExecuteBatchWithCallback(value)
        }
    }
    impl ::core::convert::From<ExecuteWithCallbackCall> for PriorityOrderReactorCalls {
        fn from(value: ExecuteWithCallbackCall) -> Self {
            Self::ExecuteWithCallback(value)
        }
    }
    impl ::core::convert::From<FeeControllerCall> for PriorityOrderReactorCalls {
        fn from(value: FeeControllerCall) -> Self {
            Self::FeeController(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PriorityOrderReactorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<Permit2Call> for PriorityOrderReactorCalls {
        fn from(value: Permit2Call) -> Self {
            Self::Permit2(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeControllerCall>
    for PriorityOrderReactorCalls {
        fn from(value: SetProtocolFeeControllerCall) -> Self {
            Self::SetProtocolFeeController(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PriorityOrderReactorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `feeController` function with signature `feeController()` and selector `0x6999b377`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeeControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `permit2` function with signature `permit2()` and selector `0x12261ee7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Permit2Return(pub ::ethers::core::types::Address);
}
