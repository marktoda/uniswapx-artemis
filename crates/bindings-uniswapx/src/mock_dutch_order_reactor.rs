pub use mock_dutch_order_reactor::*;
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
pub mod mock_dutch_order_reactor {
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
                    ::std::borrow::ToOwned::to_owned("resolveOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resolveOrder"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("resolvedOrder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
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
                    ::std::borrow::ToOwned::to_owned("DeadlineBeforeEndTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DeadlineBeforeEndTime",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("EndTimeBeforeStartTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EndTimeBeforeStartTime",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("IncorrectAmounts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("IncorrectAmounts"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InputAndOutputDecay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InputAndOutputDecay",
                            ),
                            inputs: ::std::vec![],
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
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKDUTCHORDERREACTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\x000\xEF8\x03\x80b\x000\xEF\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xC0V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x84\x92\x84\x92\x84\x92\x84\x92\x83\x92\x83\x92\x90\x91\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3PP`\x01`\x02UP`\x01`\x01`\xA0\x1B\x03\x16`\x80RPb\0\0\xFF\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xBDW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xD4W`\0\x80\xFD[\x82Qb\0\0\xE1\x81b\0\0\xA7V[` \x84\x01Q\x90\x92Pb\0\0\xF4\x81b\0\0\xA7V[\x80\x91PP\x92P\x92\x90PV[`\x80Qa/\xCEb\0\x01!`\09`\0\x81\x81`\xFB\x01Ra\x14\x1A\x01Ra/\xCE`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xB5W`\x005`\xE0\x1C\x80c?b\x19.\x11a\0iW\x80c\x8D\xA5\xCB[\x11a\0NW\x80c\x8D\xA5\xCB[\x14a\x01\xBAW\x80c\x9B\xB3\"e\x14a\x01\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x14W`\0\x80\xFD[\x80c?b\x19.\x14a\x01zW\x80ci\x99\xB3w\x14a\x01\x8DW`\0\x80\xFD[\x80c\x12&\x1E\xE7\x11a\0\x9AW\x80c\x12&\x1E\xE7\x14a\0\xE9W\x80c\x13\xFBr\xC7\x14a\x01GW\x80c-w\x13\x89\x14a\x01ZW`\0\x80\xFD[\x80c\r3X\x84\x14a\0\xC1W\x80c\rz\x16\xC3\x14a\0\xD6W`\0\x80\xFD[6a\0\xBCW\0[`\0\x80\xFD[a\0\xD4a\0\xCF6`\x04a\"\xA6V[a\x024V[\0[a\0\xD4a\0\xE46`\x04a#TV[a\x03\xADV[4\x80\x15a\0\xF5W`\0\x80\xFD[Pa\x01\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD4a\x01U6`\x04a#\x96V[a\x05\x0EV[4\x80\x15a\x01fW`\0\x80\xFD[Pa\0\xD4a\x01u6`\x04a$$V[a\x06\xCCV[a\0\xD4a\x01\x886`\x04a$HV[a\x07\xD8V[4\x80\x15a\x01\x99W`\0\x80\xFD[P`\x01Ta\x01\x1D\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xC6W`\0\x80\xFD[P`\0Ta\x01\x1D\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xF3W`\0\x80\xFD[Pa\x02\x07a\x02\x026`\x04a$HV[a\x08\xDDV[`@Qa\x01>\x91\x90a&QV[4\x80\x15a\x02 W`\0\x80\xFD[Pa\0\xD4a\x02/6`\x04a$$V[a\t[V[a\x02<a\nLV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x02SW\x90PP\x90Pa\x02\xFB\x84a\n\xBDV[\x81`\0\x81Q\x81\x10a\x03\x0EWa\x03\x0Ea&\x93V[` \x02` \x01\x01\x81\x90RPa\x03\"\x81a\x0B5V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x03b\x90\x84\x90\x87\x90\x87\x90`\x04\x01a&\xC2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03|W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x90W=`\0\x80>=`\0\xFD[PPPPa\x03\x9D\x81a\x0B\x86V[Pa\x03\xA8`\x01`\x02UV[PPPV[a\x03\xB5a\nLV[\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xD1Wa\x03\xD1a&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\x8CW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x03\xEFW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x04\xEBWa\x04\xC6\x85\x85\x83\x81\x81\x10a\x04\xAFWa\x04\xAFa&\x93V[\x90P` \x02\x81\x01\x90a\x04\xC1\x91\x90a'\x88V[a\n\xBDV[\x82\x82\x81Q\x81\x10a\x04\xD8Wa\x04\xD8a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\x92V[Pa\x04\xF5\x81a\x0B5V[a\x04\xFE\x81a\x0B\x86V[PPa\x05\n`\x01`\x02UV[PPV[a\x05\x16a\nLV[\x82`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x052Wa\x052a&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xEDW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x05PW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x065Wa\x06\x10\x87\x87\x83\x81\x81\x10a\x04\xAFWa\x04\xAFa&\x93V[\x82\x82\x81Q\x81\x10a\x06\"Wa\x06\"a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xF3V[Pa\x06?\x81a\x0B5V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x06\x7F\x90\x84\x90\x88\x90\x88\x90`\x04\x01a&\xC2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xADW=`\0\x80>=`\0\xFD[PPPPa\x06\xBA\x81a\x0B\x86V[PPa\x06\xC6`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07RW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x07\xE0a\nLV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x07\xF7W\x90PP\x90Pa\x08\x9F\x82a\n\xBDV[\x81`\0\x81Q\x81\x10a\x08\xB2Wa\x08\xB2a&\x93V[` \x02` \x01\x01\x81\x90RPa\x08\xC6\x81a\x0B5V[a\x08\xCF\x81a\x0B\x86V[Pa\x08\xDA`\x01`\x02UV[PV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\tU\x82a\n\xBDV[\x92\x91PPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07IV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\n\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07IV[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\tU\x82a\x0C\xD9V[\x80Q`\0[\x81\x81\x10\x15a\x03\xA8W`\0\x83\x82\x81Q\x81\x10a\x0BVWa\x0BVa&\x93V[` \x02` \x01\x01Q\x90Pa\x0Bi\x81a\x0E'V[a\x0Bs\x813a\x13\x17V[a\x0B}\x813a\x14\x18V[P`\x01\x01a\x0B:V[\x80Q`\0[\x81\x81\x10\x15a\x0C\xC8W`\0\x83\x82\x81Q\x81\x10a\x0B\xA7Wa\x0B\xA7a&\x93V[` \x02` \x01\x01Q\x90P`\0\x81`@\x01QQ\x90P`\0[\x81\x81\x10\x15a\x0C(W`\0\x83`@\x01Q\x82\x81Q\x81\x10a\x0B\xDEWa\x0B\xDEa&\x93V[` \x02` \x01\x01Q\x90Pa\x0C\x1F\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17\xBE\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x0B\xBEV[P\x81`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x85\x81Q\x81\x10a\x0CqWa\x0Cqa&\x93V[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x85`\0\x01Q`@\x01Q`@Qa\x0C\xB6\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x01a\x0B\x8BV[PG\x15a\x05\nWa\x05\n3Ga\x18\x05V[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\rR\x83\x80a'\xC6V[\x81\x01\x90a\r_\x91\x90a*\xB8V[\x90Pa\rj\x81a\x18\x9FV[`@Q\x80`\xA0\x01`@R\x80\x82`\0\x01Q\x81R` \x01a\r\xA0\x83` \x01Q\x84`@\x01Q\x85``\x01Qa\x19\x8A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\r\xC6\x83` \x01Q\x84`@\x01Q\x85`\x80\x01Qa\x1A\\\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01\x84\x80` \x01\x90a\r\xDB\x91\x90a'\xC6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x0E\x1E\x83a\x1BCV[\x90R\x93\x92PPPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0EGWPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x0E\x9E\x90\x85\x90`\x04\x01a&QV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0F\x01\x91\x90\x81\x01\x90a+\xECV[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x0F\x19\x82\x84a,\xBCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F1Wa\x0F1a&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x9AW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x0FOW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0F\xEBW\x85`@\x01Q\x81\x81Q\x81\x10a\x0F\xBEWa\x0F\xBEa&\x93V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0F\xD8Wa\x0F\xD8a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\xA0V[P`\0\x80`\0[\x84\x81\x10\x15a\x13\x06W`\0\x87\x82\x81Q\x81\x10a\x10\x0EWa\x10\x0Ea&\x93V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x10\xCCW\x88\x81\x81Q\x81\x10a\x105Wa\x105a&\x93V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x10\xC4W\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07IV[`\x01\x01a\x10\x1BV[P`\0\x80[\x88\x81\x10\x15a\x11\x8DW`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x10\xF1Wa\x10\xF1a&\x93V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11\x84W\x85\x15a\x11nW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x11}\x90\x84a,\xBCV[\x92P`\x01\x96P[P`\x01\x01a\x10\xD1V[P\x81Q` \x8B\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x12\x06W\x84\x15a\x11\xEDW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x8B\x01Q\x01Qa\x11\xFF\x90\x82a,\xBCV[\x90P`\x01\x93P[\x80`\0\x03a\x12[W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07IV[a\x12i\x81`\x05a'\x10a\x1D\xADV[\x82` \x01Q\x11\x15a\x12\xDCW\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x07IV[\x81\x86\x84\x8A\x01\x81Q\x81\x10a\x12\xF1Wa\x12\xF1a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0F\xF2V[PPP`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x13hW`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x05\nW\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x13\xE4\x90\x84\x90\x86\x90`\x04\x01a,\xF6V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xFCW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\x10W=`\0\x80>=`\0\xFD[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x14\xD8\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82R\x80\x87\x01Q\x81\x01Q\x90\x82\x01R\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q` \x01a\x16c\x90\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`\x1A\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`1\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`F\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`Y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`r\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\x89\x82\x01R`\x9F\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.\x8C` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a/\x0C`\x8D\x919`@Q` \x01a\x16\xD0\x93\x92\x91\x90a-%V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R``\x83\x01\x90\x91R`.\x80\x83R\x90\x91\x90a.\xDE` \x83\x019`@Q` \x01a\x17#\x92\x91\x90a-hV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x17\x90\x96\x95\x94\x93\x92`\x04\x01a-\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x10W=`\0\x80>=`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x17\xE3Wa\x03\xA8\x82\x82a\x18\x05V[a\x03\xA8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x84\x84a\x1D\xE9V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18dV[``\x91P[PP\x90P\x80a\x03\xA8W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Q\x81Q``\x01Q\x10\x15a\x18\xE2W`@Q\x7Fw:a\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`@\x81\x01Q` \x90\x91\x01Q\x14a\x08\xDAW`\0[\x81`\x80\x01QQ\x81\x10\x15a\x05\nW\x81`\x80\x01Q\x81\x81Q\x81\x10a\x19\x1DWa\x19\x1Da&\x93V[` \x02` \x01\x01Q`@\x01Q\x82`\x80\x01Q\x82\x81Q\x81\x10a\x19?Wa\x19?a&\x93V[` \x02` \x01\x01Q` \x01Q\x14a\x19\x82W`@Q\x7F\xD3\x03u\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x18\xFAV[a\x19\xC4`@Q\x80``\x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x83`@\x01Q\x84` \x01Q\x11\x15a\x1A\x06W`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1A\x1C\x85` \x01Q\x86`@\x01Q\x86\x86a\x1E\xDBV[`@\x80Q``\x81\x01\x82R\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x92\x90\x92R\x95\x86\x01Q\x95\x81\x01\x95\x90\x95RP\x92\x93\x92PPPV[\x82Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AzWa\x1Aza&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xE3W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x1A\x98W\x90P[P\x91P`\0[\x81\x81\x10\x15a\x1B:Wa\x1B\x15\x86\x82\x81Q\x81\x10a\x1B\x06Wa\x1B\x06a&\x93V[` \x02` \x01\x01Q\x86\x86a\x1F\x82V[\x83\x82\x81Q\x81\x10a\x1B'Wa\x1B'a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xE9V[PP\x93\x92PPPV[`@Q\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`+\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`:\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`Q\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`f\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`\x92\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\xA9\x82\x01R`\0\x90`\xBF\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.\x8C` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a/\x0C`\x8D\x919`@Q` \x01a\x1C\xE9\x93\x92\x91\x90a-%V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1D\x0C\x83`\0\x01Qa RV[\x83` \x01Q\x84`@\x01Q\x85``\x01Q`\0\x01Q\x86``\x01Q` \x01Q\x87``\x01Q`@\x01Qa\x1D>\x89`\x80\x01Qa \xECV[`@\x80Q` \x81\x01\x99\x90\x99R\x88\x01\x96\x90\x96R``\x87\x01\x94\x90\x94R`\x80\x86\x01\x92\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x1D\xE2W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x1E\xD4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07IV[PPPPPV[`\0\x83\x85\x03a\x1E\xEBWP\x83a\x1FzV[\x82\x82\x11a\x1F$W`@Q\x7FC\x134S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x82\x11a\x1F2WP\x82a\x1FzV[B\x83\x10a\x1F@WP\x83a\x1FzV[B\x83\x90\x03\x83\x83\x03\x86\x86\x10\x15a\x1FeWa\x1F\\\x86\x88\x03\x83\x83a\x1D\xADV[\x87\x03\x92Pa\x1FwV[a\x1Fr\x87\x87\x03\x83\x83a!\x8AV[\x87\x01\x92P[PP[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x83`@\x01Q\x84` \x01Q\x10\x15a\x1F\xE1W`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1F\xF7\x85` \x01Q\x86`@\x01Q\x86\x86a\x1E\xDBV[\x90P`@Q\x80``\x01`@R\x80\x86`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81R` \x01\x86``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x91PP\x93\x92PPPV[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a/\x0C`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\x1D\x90\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\x0CWa!\x0Ca&dV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a!{W`\0a!i\x85\x83\x81Q\x81\x10a!\\Wa!\\a&\x93V[` \x02` \x01\x01Qa!\xCEV[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a!<V[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a!\xBFW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0`@Q\x80`\x80\x01`@R\x80`R\x81R` \x01a.\x8C`R\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qa\x1D\x90\x96\x91\x92\x91\x01\x94\x85Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01R`@\x85\x01\x92\x90\x92R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\0`@\x82\x84\x03\x12\x15a\"WW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\"oW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x87W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"\x9FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\xBBW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD3W`\0\x80\xFD[a\"\xDF\x87\x83\x88\x01a\"EV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\"\xF5W`\0\x80\xFD[Pa#\x02\x86\x82\x87\x01a\"]V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a#!W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"\x9FW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a#gW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#~W`\0\x80\xFD[a#\x8A\x85\x82\x86\x01a#\x0FV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a#\xACW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\xC4W`\0\x80\xFD[a#\xD0\x88\x83\x89\x01a#\x0FV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a#\xE9W`\0\x80\xFD[Pa#\xF6\x87\x82\x88\x01a\"]V[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$6W`\0\x80\xFD[\x815a$A\x81a$\x02V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a$ZW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$qW`\0\x80\xFD[a\x1Fz\x84\x82\x85\x01a\"EV[`\0[\x83\x81\x10\x15a$\x98W\x81\x81\x01Q\x83\x82\x01R` \x01a$\x80V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\xB9\x81` \x86\x01` \x86\x01a$}V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a%KW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a%\0V[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra%\xCAa\x01\xA0\x85\x01\x82a$\xA1V[\x90P` \x83\x01Qa&\x08` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra& \x82\x82a$\xEBV[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra&:\x82\x82a$\xA1V[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a$A` \x83\x01\x84a%VV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x01`@\x83R\x80\x86Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x89\x01`\0[\x83\x81\x10\x15a'7W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x85Ra'%\x86\x83Qa%VV[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a&\xEBV[PP\x85\x84\x03\x81\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x96\x95PPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a'\xBCW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a'\xFBW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\x16W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\"\x9FW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\x04Wa)\x04a&dV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a)\x1DW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)7Wa)7a&dV[a)h` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a(\xBDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a)}W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a)\xACW`\0\x80\xFD[a)\xB4a(+V[\x90P\x815a)\xC1\x81a$\x02V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\xF9Wa)\xF9a&dV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a*\x14W`\0\x80\xFD[\x815` a*)a*$\x83a)\xDFV[a(\xBDV[\x82\x81R`\x07\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a*HW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a*\xADW`\x80\x81\x89\x03\x12\x15a*eW`\0\x80\x81\xFD[a*ma(TV[\x815a*x\x81a$\x02V[\x81R\x81\x85\x015\x85\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015a*\x9B\x81a$\x02V[\x90\x82\x01R\x83R\x91\x83\x01\x91`\x80\x01a*LV[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a*\xCAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*\xE2W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a*\xF6W`\0\x80\xFD[a*\xFEa(wV[\x825\x82\x81\x11\x15a+\rW`\0\x80\xFD[\x83\x01`\xC0\x81\x88\x03\x12\x15a+\x1FW`\0\x80\xFD[a+'a(\x9AV[\x815a+2\x81a$\x02V[\x81R` \x82\x015a+B\x81a$\x02V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a+k\x81a$\x02V[`\x80\x82\x01R`\xA0\x82\x015\x84\x81\x11\x15a+\x82W`\0\x80\xFD[a+\x8E\x89\x82\x85\x01a)\x0CV[`\xA0\x83\x01RP\x80\x83RPP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra+\xBA\x86``\x85\x01a)\x9AV[``\x82\x01R`\xC0\x83\x015\x82\x81\x11\x15a+\xD1W`\0\x80\xFD[a+\xDD\x87\x82\x86\x01a*\x03V[`\x80\x83\x01RP\x95\x94PPPPPV[`\0` \x80\x83\x85\x03\x12\x15a+\xFFW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x16W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a,'W`\0\x80\xFD[\x80Qa,5a*$\x82a)\xDFV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a,TW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a,\xB0W\x80\x85\x8A\x03\x12\x15a,qW`\0\x80\x81\xFD[a,ya(+V[\x85Qa,\x84\x81a$\x02V[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa,\x9D\x81a$\x02V[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a,YV[P\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\tUW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x1Fz`@\x83\x01\x84a%VV[`\0\x84Qa-7\x81\x84` \x89\x01a$}V[\x84Q\x90\x83\x01\x90a-K\x81\x83` \x89\x01a$}V[\x84Q\x91\x01\x90a-^\x81\x83` \x88\x01a$}V[\x01\x95\x94PPPPPV[\x7FDutchOrder witness)\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa-\xA0\x81`\x13\x85\x01` \x88\x01a$}V[\x83Q\x90\x83\x01\x90a-\xB7\x81`\x13\x84\x01` \x88\x01a$}V[\x01`\x13\x01\x94\x93PPPPV[`\0a\x01@a-\xF3\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra.4`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra.i\x81\x84\x01\x86a$\xA1V[\x90P\x82\x81\x03a\x01 \x84\x01Ra.~\x81\x85a$\xA1V[\x99\x98PPPPPPPPPV\xFEDutchOutput(address token,uint256 startAmount,uint256 endAmount,address recipient)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)\xA2dipfsX\"\x12 \xAB\x07\xF1\x02\xE92K\x13\x82\x92h\xE6\0Mj\x98'q\xC1&\x87h]3\xAFh\xC0\xA0\x89>\xD6\x81dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKDUTCHORDERREACTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xB5W`\x005`\xE0\x1C\x80c?b\x19.\x11a\0iW\x80c\x8D\xA5\xCB[\x11a\0NW\x80c\x8D\xA5\xCB[\x14a\x01\xBAW\x80c\x9B\xB3\"e\x14a\x01\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x14W`\0\x80\xFD[\x80c?b\x19.\x14a\x01zW\x80ci\x99\xB3w\x14a\x01\x8DW`\0\x80\xFD[\x80c\x12&\x1E\xE7\x11a\0\x9AW\x80c\x12&\x1E\xE7\x14a\0\xE9W\x80c\x13\xFBr\xC7\x14a\x01GW\x80c-w\x13\x89\x14a\x01ZW`\0\x80\xFD[\x80c\r3X\x84\x14a\0\xC1W\x80c\rz\x16\xC3\x14a\0\xD6W`\0\x80\xFD[6a\0\xBCW\0[`\0\x80\xFD[a\0\xD4a\0\xCF6`\x04a\"\xA6V[a\x024V[\0[a\0\xD4a\0\xE46`\x04a#TV[a\x03\xADV[4\x80\x15a\0\xF5W`\0\x80\xFD[Pa\x01\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD4a\x01U6`\x04a#\x96V[a\x05\x0EV[4\x80\x15a\x01fW`\0\x80\xFD[Pa\0\xD4a\x01u6`\x04a$$V[a\x06\xCCV[a\0\xD4a\x01\x886`\x04a$HV[a\x07\xD8V[4\x80\x15a\x01\x99W`\0\x80\xFD[P`\x01Ta\x01\x1D\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xC6W`\0\x80\xFD[P`\0Ta\x01\x1D\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x01\xF3W`\0\x80\xFD[Pa\x02\x07a\x02\x026`\x04a$HV[a\x08\xDDV[`@Qa\x01>\x91\x90a&QV[4\x80\x15a\x02 W`\0\x80\xFD[Pa\0\xD4a\x02/6`\x04a$$V[a\t[V[a\x02<a\nLV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x02SW\x90PP\x90Pa\x02\xFB\x84a\n\xBDV[\x81`\0\x81Q\x81\x10a\x03\x0EWa\x03\x0Ea&\x93V[` \x02` \x01\x01\x81\x90RPa\x03\"\x81a\x0B5V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x03b\x90\x84\x90\x87\x90\x87\x90`\x04\x01a&\xC2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03|W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x90W=`\0\x80>=`\0\xFD[PPPPa\x03\x9D\x81a\x0B\x86V[Pa\x03\xA8`\x01`\x02UV[PPPV[a\x03\xB5a\nLV[\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xD1Wa\x03\xD1a&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\x8CW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x03\xEFW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x04\xEBWa\x04\xC6\x85\x85\x83\x81\x81\x10a\x04\xAFWa\x04\xAFa&\x93V[\x90P` \x02\x81\x01\x90a\x04\xC1\x91\x90a'\x88V[a\n\xBDV[\x82\x82\x81Q\x81\x10a\x04\xD8Wa\x04\xD8a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\x92V[Pa\x04\xF5\x81a\x0B5V[a\x04\xFE\x81a\x0B\x86V[PPa\x05\n`\x01`\x02UV[PPV[a\x05\x16a\nLV[\x82`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x052Wa\x052a&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xEDW\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x05PW\x90P[P\x90P`\0[\x82\x81\x10\x15a\x065Wa\x06\x10\x87\x87\x83\x81\x81\x10a\x04\xAFWa\x04\xAFa&\x93V[\x82\x82\x81Q\x81\x10a\x06\"Wa\x06\"a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xF3V[Pa\x06?\x81a\x0B5V[`@Q\x7FX]\xA6(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90cX]\xA6(\x90a\x06\x7F\x90\x84\x90\x88\x90\x88\x90`\x04\x01a&\xC2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xADW=`\0\x80>=`\0\xFD[PPPPa\x06\xBA\x81a\x0B\x86V[PPa\x06\xC6`\x01`\x02UV[PPPPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x07RW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[a\x07\xE0a\nLV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x80\x85\x01\x91\x90\x91R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x07\xF7W\x90PP\x90Pa\x08\x9F\x82a\n\xBDV[\x81`\0\x81Q\x81\x10a\x08\xB2Wa\x08\xB2a&\x93V[` \x02` \x01\x01\x81\x90RPa\x08\xC6\x81a\x0B5V[a\x08\xCF\x81a\x0B\x86V[Pa\x08\xDA`\x01`\x02UV[PV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\tU\x82a\n\xBDV[\x92\x91PPV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\t\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07IV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x02\x80T\x03a\n\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07IV[`\x02\x80UV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\tU\x82a\x0C\xD9V[\x80Q`\0[\x81\x81\x10\x15a\x03\xA8W`\0\x83\x82\x81Q\x81\x10a\x0BVWa\x0BVa&\x93V[` \x02` \x01\x01Q\x90Pa\x0Bi\x81a\x0E'V[a\x0Bs\x813a\x13\x17V[a\x0B}\x813a\x14\x18V[P`\x01\x01a\x0B:V[\x80Q`\0[\x81\x81\x10\x15a\x0C\xC8W`\0\x83\x82\x81Q\x81\x10a\x0B\xA7Wa\x0B\xA7a&\x93V[` \x02` \x01\x01Q\x90P`\0\x81`@\x01QQ\x90P`\0[\x81\x81\x10\x15a\x0C(W`\0\x83`@\x01Q\x82\x81Q\x81\x10a\x0B\xDEWa\x0B\xDEa&\x93V[` \x02` \x01\x01Q\x90Pa\x0C\x1F\x81`@\x01Q\x82` \x01Q\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17\xBE\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P`\x01\x01a\x0B\xBEV[P\x81`\0\x01Q` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x85\x81Q\x81\x10a\x0CqWa\x0Cqa&\x93V[` \x02` \x01\x01Q`\x80\x01Q\x7Fx\xAD~\xC0\xE9\xF8\x9Et\x01*\xFAXs\x8Bkf\x1C\x02L\xB0\xFD\x18^\xE2\xF6\x16\xC0\xA2\x89$\xBDf\x85`\0\x01Q`@\x01Q`@Qa\x0C\xB6\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x01a\x0B\x8BV[PG\x15a\x05\nWa\x05\n3Ga\x18\x05V[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x82\x90R\x90a\rR\x83\x80a'\xC6V[\x81\x01\x90a\r_\x91\x90a*\xB8V[\x90Pa\rj\x81a\x18\x9FV[`@Q\x80`\xA0\x01`@R\x80\x82`\0\x01Q\x81R` \x01a\r\xA0\x83` \x01Q\x84`@\x01Q\x85``\x01Qa\x19\x8A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\r\xC6\x83` \x01Q\x84`@\x01Q\x85`\x80\x01Qa\x1A\\\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01\x84\x80` \x01\x90a\r\xDB\x91\x90a'\xC6V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x0E\x1E\x83a\x1BCV[\x90R\x93\x92PPPV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0EGWPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x0E\x9E\x90\x85\x90`\x04\x01a&QV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x0F\x01\x91\x90\x81\x01\x90a+\xECV[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x0F\x19\x82\x84a,\xBCV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F1Wa\x0F1a&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x9AW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x0FOW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0F\xEBW\x85`@\x01Q\x81\x81Q\x81\x10a\x0F\xBEWa\x0F\xBEa&\x93V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0F\xD8Wa\x0F\xD8a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\xA0V[P`\0\x80`\0[\x84\x81\x10\x15a\x13\x06W`\0\x87\x82\x81Q\x81\x10a\x10\x0EWa\x10\x0Ea&\x93V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x10\xCCW\x88\x81\x81Q\x81\x10a\x105Wa\x105a&\x93V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x10\xC4W\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07IV[`\x01\x01a\x10\x1BV[P`\0\x80[\x88\x81\x10\x15a\x11\x8DW`\0\x8B`@\x01Q\x82\x81Q\x81\x10a\x10\xF1Wa\x10\xF1a&\x93V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x11\x84W\x85\x15a\x11nW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x11}\x90\x84a,\xBCV[\x92P`\x01\x96P[P`\x01\x01a\x10\xD1V[P\x81Q` \x8B\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x12\x06W\x84\x15a\x11\xEDW`@Q\x7F\xED\xC7\xE2\xE4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x8B\x01Q\x01Qa\x11\xFF\x90\x82a,\xBCV[\x90P`\x01\x93P[\x80`\0\x03a\x12[W\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x07IV[a\x12i\x81`\x05a'\x10a\x1D\xADV[\x82` \x01Q\x11\x15a\x12\xDCW\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x07IV[\x81\x86\x84\x8A\x01\x81Q\x81\x10a\x12\xF1Wa\x12\xF1a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0F\xF2V[PPP`@\x90\x94\x01\x93\x90\x93RPPPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\x13hW`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x05\nW\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x13\xE4\x90\x84\x90\x86\x90`\x04\x01a,\xF6V[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x13\xFCW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14\x10W=`\0\x80>=`\0\xFD[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x13|)\xFEa\x14\xD8\x84`@\x80Q`\xA0\x81\x01\x82R`\0``\x82\x01\x81\x81R`\x80\x83\x01\x82\x90R\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R` \x80\x84\x01\x80QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x80\x85\x01\x91\x82R\x91Q\x85\x01Q`\x80\x85\x01R\x83R\x84Q\x84\x01Q\x91\x83\x01\x91\x90\x91R\x92Q\x90\x92\x01Q\x90\x82\x01R\x90V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82R\x80\x87\x01Q\x81\x01Q\x90\x82\x01R\x85`\0\x01Q` \x01Q\x86`\x80\x01Q`@Q` \x01a\x16c\x90\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`\x1A\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`1\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`F\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`Y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`r\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\x89\x82\x01R`\x9F\x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.\x8C` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a/\x0C`\x8D\x919`@Q` \x01a\x16\xD0\x93\x92\x91\x90a-%V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R``\x83\x01\x90\x91R`.\x80\x83R\x90\x91\x90a.\xDE` \x83\x019`@Q` \x01a\x17#\x92\x91\x90a-hV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R``\x8A\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x83Ra\x17\x90\x96\x95\x94\x93\x92`\x04\x01a-\xC3V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xAAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x10W=`\0\x80>=`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x17\xE3Wa\x03\xA8\x82\x82a\x18\x05V[a\x03\xA8s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x163\x84\x84a\x1D\xE9V[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18dV[``\x91P[PP\x90P\x80a\x03\xA8W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x01Q\x81Q``\x01Q\x10\x15a\x18\xE2W`@Q\x7Fw:a\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x81\x01Q`@\x81\x01Q` \x90\x91\x01Q\x14a\x08\xDAW`\0[\x81`\x80\x01QQ\x81\x10\x15a\x05\nW\x81`\x80\x01Q\x81\x81Q\x81\x10a\x19\x1DWa\x19\x1Da&\x93V[` \x02` \x01\x01Q`@\x01Q\x82`\x80\x01Q\x82\x81Q\x81\x10a\x19?Wa\x19?a&\x93V[` \x02` \x01\x01Q` \x01Q\x14a\x19\x82W`@Q\x7F\xD3\x03u\x8B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x18\xFAV[a\x19\xC4`@Q\x80``\x01`@R\x80`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x83`@\x01Q\x84` \x01Q\x11\x15a\x1A\x06W`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1A\x1C\x85` \x01Q\x86`@\x01Q\x86\x86a\x1E\xDBV[`@\x80Q``\x81\x01\x82R\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x92\x90\x92R\x95\x86\x01Q\x95\x81\x01\x95\x90\x95RP\x92\x93\x92PPPV[\x82Q``\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AzWa\x1Aza&dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xE3W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x1A\x98W\x90P[P\x91P`\0[\x81\x81\x10\x15a\x1B:Wa\x1B\x15\x86\x82\x81Q\x81\x10a\x1B\x06Wa\x1B\x06a&\x93V[` \x02` \x01\x01Q\x86\x86a\x1F\x82V[\x83\x82\x81Q\x81\x10a\x1B'Wa\x1B'a&\x93V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xE9V[PP\x93\x92PPPV[`@Q\x7FDutchOrder(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7FOrderInfo info,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`+\x82\x01R\x7Fuint256 decayStartTime,\0\0\0\0\0\0\0\0\0`:\x82\x01R\x7Fuint256 decayEndTime,\0\0\0\0\0\0\0\0\0\0\0`Q\x82\x01R\x7Faddress inputToken,\0\0\0\0\0\0\0\0\0\0\0\0\0`f\x82\x01R\x7Fuint256 inputStartAmount,\0\0\0\0\0\0\0`y\x82\x01R\x7Fuint256 inputEndAmount,\0\0\0\0\0\0\0\0\0`\x92\x82\x01R\x7FDutchOutput[] outputs)\0\0\0\0\0\0\0\0\0\0`\xA9\x82\x01R`\0\x90`\xBF\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R`\x80\x83\x01\x90\x91R`R\x80\x83R\x90\x91\x90a.\x8C` \x83\x019`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a/\x0C`\x8D\x919`@Q` \x01a\x1C\xE9\x93\x92\x91\x90a-%V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1D\x0C\x83`\0\x01Qa RV[\x83` \x01Q\x84`@\x01Q\x85``\x01Q`\0\x01Q\x86``\x01Q` \x01Q\x87``\x01Q`@\x01Qa\x1D>\x89`\x80\x01Qa \xECV[`@\x80Q` \x81\x01\x99\x90\x99R\x88\x01\x96\x90\x96R``\x87\x01\x94\x90\x94R`\x80\x86\x01\x92\x90\x92Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x1D\xE2W`\0\x80\xFD[P\x91\x02\x04\x90V[`\0`@Q\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x1E\xD4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07IV[PPPPPV[`\0\x83\x85\x03a\x1E\xEBWP\x83a\x1FzV[\x82\x82\x11a\x1F$W`@Q\x7FC\x134S\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x82\x11a\x1F2WP\x82a\x1FzV[B\x83\x10a\x1F@WP\x83a\x1FzV[B\x83\x90\x03\x83\x83\x03\x86\x86\x10\x15a\x1FeWa\x1F\\\x86\x88\x03\x83\x83a\x1D\xADV[\x87\x03\x92Pa\x1FwV[a\x1Fr\x87\x87\x03\x83\x83a!\x8AV[\x87\x01\x92P[PP[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x83`@\x01Q\x84` \x01Q\x10\x15a\x1F\xE1W`@Q\x7F|\x1F\x81\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1F\xF7\x85` \x01Q\x86`@\x01Q\x86\x86a\x1E\xDBV[\x90P`@Q\x80``\x01`@R\x80\x86`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81R` \x01\x86``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x91PP\x93\x92PPPV[`\0`@Q\x80`\xC0\x01`@R\x80`\x8D\x81R` \x01a/\x0C`\x8D\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x8A\x01Q\x80Q\x90\x89\x01 \x93Qa\x1D\x90\x98\x93\x94\x92\x93\x91\x92\x91\x01\x96\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x16` \x88\x01R\x93\x85\x16`@\x87\x01R``\x86\x01\x92\x90\x92R`\x80\x85\x01R\x90\x91\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01\x90V[`\0\x80\x82Q` \x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\x0CWa!\x0Ca&dV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a!{W`\0a!i\x85\x83\x81Q\x81\x10a!\\Wa!\\a&\x93V[` \x02` \x01\x01Qa!\xCEV[` \x83\x81\x02\x85\x01\x01RP`\x01\x01a!<V[P\x80Q` \x90\x91\x01 \x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a!\xBFW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[`\0`@Q\x80`\x80\x01`@R\x80`R\x81R` \x01a.\x8C`R\x919\x80Q` \x91\x82\x01 \x83Q\x84\x83\x01Q`@\x80\x87\x01Q``\x88\x01Q\x91Qa\x1D\x90\x96\x91\x92\x91\x01\x94\x85Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16` \x86\x01R`@\x85\x01\x92\x90\x92R``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`\0`@\x82\x84\x03\x12\x15a\"WW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\"oW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x87W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"\x9FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\xBBW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\"\xD3W`\0\x80\xFD[a\"\xDF\x87\x83\x88\x01a\"EV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a\"\xF5W`\0\x80\xFD[Pa#\x02\x86\x82\x87\x01a\"]V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a#!W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"\x9FW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a#gW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#~W`\0\x80\xFD[a#\x8A\x85\x82\x86\x01a#\x0FV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a#\xACW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\xC4W`\0\x80\xFD[a#\xD0\x88\x83\x89\x01a#\x0FV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a#\xE9W`\0\x80\xFD[Pa#\xF6\x87\x82\x88\x01a\"]V[\x95\x98\x94\x97P\x95PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$6W`\0\x80\xFD[\x815a$A\x81a$\x02V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a$ZW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$qW`\0\x80\xFD[a\x1Fz\x84\x82\x85\x01a\"EV[`\0[\x83\x81\x10\x15a$\x98W\x81\x81\x01Q\x83\x82\x01R` \x01a$\x80V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\xB9\x81` \x86\x01` \x86\x01a$}V[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a%KW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a%\0V[P\x94\x95\x94PPPPPV[`\0\x81Q`\xE0\x84Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16`\xE0\x86\x01R\x80` \x83\x01Q\x16a\x01\0\x86\x01R`@\x82\x01Qa\x01 \x86\x01R``\x82\x01Qa\x01@\x86\x01R\x80`\x80\x83\x01Q\x16a\x01`\x86\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\x80\x85\x01Ra%\xCAa\x01\xA0\x85\x01\x82a$\xA1V[\x90P` \x83\x01Qa&\x08` \x86\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra& \x82\x82a$\xEBV[\x91PP``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Ra&:\x82\x82a$\xA1V[\x91PP`\x80\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a$A` \x83\x01\x84a%VV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x01`@\x83R\x80\x86Q\x80\x83R``\x85\x01\x91P``\x81`\x05\x1B\x86\x01\x01\x92P` \x80\x89\x01`\0[\x83\x81\x10\x15a'7W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x88\x87\x03\x01\x85Ra'%\x86\x83Qa%VV[\x95P\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a&\xEBV[PP\x85\x84\x03\x81\x87\x01R\x86\x84R\x86\x88\x82\x86\x017`\0\x84\x88\x01\x82\x01R`\x1F\x90\x96\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x90\x92\x01\x90\x94\x01\x96\x95PPPPPPV[`\0\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x836\x03\x01\x81\x12a'\xBCW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a'\xFBW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\x16W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\"\x9FW`\0\x80\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(NWa(Na&dV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\x04Wa)\x04a&dV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a)\x1DW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)7Wa)7a&dV[a)h` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a(\xBDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a)}W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a)\xACW`\0\x80\xFD[a)\xB4a(+V[\x90P\x815a)\xC1\x81a$\x02V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\xF9Wa)\xF9a&dV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a*\x14W`\0\x80\xFD[\x815` a*)a*$\x83a)\xDFV[a(\xBDV[\x82\x81R`\x07\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a*HW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a*\xADW`\x80\x81\x89\x03\x12\x15a*eW`\0\x80\x81\xFD[a*ma(TV[\x815a*x\x81a$\x02V[\x81R\x81\x85\x015\x85\x82\x01R`@\x80\x83\x015\x90\x82\x01R``\x80\x83\x015a*\x9B\x81a$\x02V[\x90\x82\x01R\x83R\x91\x83\x01\x91`\x80\x01a*LV[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a*\xCAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*\xE2W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a*\xF6W`\0\x80\xFD[a*\xFEa(wV[\x825\x82\x81\x11\x15a+\rW`\0\x80\xFD[\x83\x01`\xC0\x81\x88\x03\x12\x15a+\x1FW`\0\x80\xFD[a+'a(\x9AV[\x815a+2\x81a$\x02V[\x81R` \x82\x015a+B\x81a$\x02V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a+k\x81a$\x02V[`\x80\x82\x01R`\xA0\x82\x015\x84\x81\x11\x15a+\x82W`\0\x80\xFD[a+\x8E\x89\x82\x85\x01a)\x0CV[`\xA0\x83\x01RP\x80\x83RPP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01Ra+\xBA\x86``\x85\x01a)\x9AV[``\x82\x01R`\xC0\x83\x015\x82\x81\x11\x15a+\xD1W`\0\x80\xFD[a+\xDD\x87\x82\x86\x01a*\x03V[`\x80\x83\x01RP\x95\x94PPPPPV[`\0` \x80\x83\x85\x03\x12\x15a+\xFFW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x16W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a,'W`\0\x80\xFD[\x80Qa,5a*$\x82a)\xDFV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a,TW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a,\xB0W\x80\x85\x8A\x03\x12\x15a,qW`\0\x80\x81\xFD[a,ya(+V[\x85Qa,\x84\x81a$\x02V[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa,\x9D\x81a$\x02V[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a,YV[P\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\tUW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0a\x1Fz`@\x83\x01\x84a%VV[`\0\x84Qa-7\x81\x84` \x89\x01a$}V[\x84Q\x90\x83\x01\x90a-K\x81\x83` \x89\x01a$}V[\x84Q\x91\x01\x90a-^\x81\x83` \x88\x01a$}V[\x01\x95\x94PPPPPV[\x7FDutchOrder witness)\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa-\xA0\x81`\x13\x85\x01` \x88\x01a$}V[\x83Q\x90\x83\x01\x90a-\xB7\x81`\x13\x84\x01` \x88\x01a$}V[\x01`\x13\x01\x94\x93PPPPV[`\0a\x01@a-\xF3\x83\x8AQ\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[` \x89\x01Q`@\x84\x01R`@\x89\x01Q``\x84\x01Ra.4`\x80\x84\x01\x89\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01Q\x91\x01RV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\xC0\x84\x01R\x85`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra.i\x81\x84\x01\x86a$\xA1V[\x90P\x82\x81\x03a\x01 \x84\x01Ra.~\x81\x85a$\xA1V[\x99\x98PPPPPPPPPV\xFEDutchOutput(address token,uint256 startAmount,uint256 endAmount,address recipient)TokenPermissions(address token,uint256 amount)OrderInfo(address reactor,address swapper,uint256 nonce,uint256 deadline,address additionalValidationContract,bytes additionalValidationData)\xA2dipfsX\"\x12 \xAB\x07\xF1\x02\xE92K\x13\x82\x92h\xE6\0Mj\x98'q\xC1&\x87h]3\xAFh\xC0\xA0\x89>\xD6\x81dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKDUTCHORDERREACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockDutchOrderReactor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockDutchOrderReactor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockDutchOrderReactor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockDutchOrderReactor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockDutchOrderReactor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockDutchOrderReactor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockDutchOrderReactor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKDUTCHORDERREACTOR_ABI.clone(),
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
                MOCKDUTCHORDERREACTOR_ABI.clone(),
                MOCKDUTCHORDERREACTOR_BYTECODE.clone().into(),
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
        ///Calls the contract's `resolveOrder` (0x9bb32265) function
        pub fn resolve_order(
            &self,
            order: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash([155, 179, 34, 101], (order,))
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
            MockDutchOrderReactorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockDutchOrderReactor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DeadlineBeforeEndTime` with signature `DeadlineBeforeEndTime()` and selector `0x773a6187`
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
    #[etherror(name = "DeadlineBeforeEndTime", abi = "DeadlineBeforeEndTime()")]
    pub struct DeadlineBeforeEndTime;
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
    ///Custom Error type `EndTimeBeforeStartTime` with signature `EndTimeBeforeStartTime()` and selector `0x43133453`
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
    #[etherror(name = "EndTimeBeforeStartTime", abi = "EndTimeBeforeStartTime()")]
    pub struct EndTimeBeforeStartTime;
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
    ///Custom Error type `IncorrectAmounts` with signature `IncorrectAmounts()` and selector `0x7c1f8113`
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
    #[etherror(name = "IncorrectAmounts", abi = "IncorrectAmounts()")]
    pub struct IncorrectAmounts;
    ///Custom Error type `InputAndOutputDecay` with signature `InputAndOutputDecay()` and selector `0xd303758b`
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
    #[etherror(name = "InputAndOutputDecay", abi = "InputAndOutputDecay()")]
    pub struct InputAndOutputDecay;
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
    pub enum MockDutchOrderReactorErrors {
        DeadlineBeforeEndTime(DeadlineBeforeEndTime),
        DuplicateFeeOutput(DuplicateFeeOutput),
        EndTimeBeforeStartTime(EndTimeBeforeStartTime),
        FeeTooLarge(FeeTooLarge),
        IncorrectAmounts(IncorrectAmounts),
        InputAndOutputDecay(InputAndOutputDecay),
        InputAndOutputFees(InputAndOutputFees),
        InvalidFeeToken(InvalidFeeToken),
        InvalidReactor(InvalidReactor),
        NativeTransferFailed(NativeTransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockDutchOrderReactorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DeadlineBeforeEndTime as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeadlineBeforeEndTime(decoded));
            }
            if let Ok(decoded) = <DuplicateFeeOutput as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DuplicateFeeOutput(decoded));
            }
            if let Ok(decoded) = <EndTimeBeforeStartTime as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EndTimeBeforeStartTime(decoded));
            }
            if let Ok(decoded) = <FeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeTooLarge(decoded));
            }
            if let Ok(decoded) = <IncorrectAmounts as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncorrectAmounts(decoded));
            }
            if let Ok(decoded) = <InputAndOutputDecay as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InputAndOutputDecay(decoded));
            }
            if let Ok(decoded) = <InputAndOutputFees as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InputAndOutputFees(decoded));
            }
            if let Ok(decoded) = <InvalidFeeToken as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidFeeToken(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockDutchOrderReactorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeadlineBeforeEndTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DuplicateFeeOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EndTimeBeforeStartTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InputAndOutputDecay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InputAndOutputFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockDutchOrderReactorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DeadlineBeforeEndTime as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DuplicateFeeOutput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EndTimeBeforeStartTime as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FeeTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <IncorrectAmounts as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InputAndOutputDecay as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InputAndOutputFees as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFeeToken as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockDutchOrderReactorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeadlineBeforeEndTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DuplicateFeeOutput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EndTimeBeforeStartTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::InputAndOutputDecay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InputAndOutputFees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidFeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockDutchOrderReactorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeadlineBeforeEndTime> for MockDutchOrderReactorErrors {
        fn from(value: DeadlineBeforeEndTime) -> Self {
            Self::DeadlineBeforeEndTime(value)
        }
    }
    impl ::core::convert::From<DuplicateFeeOutput> for MockDutchOrderReactorErrors {
        fn from(value: DuplicateFeeOutput) -> Self {
            Self::DuplicateFeeOutput(value)
        }
    }
    impl ::core::convert::From<EndTimeBeforeStartTime> for MockDutchOrderReactorErrors {
        fn from(value: EndTimeBeforeStartTime) -> Self {
            Self::EndTimeBeforeStartTime(value)
        }
    }
    impl ::core::convert::From<FeeTooLarge> for MockDutchOrderReactorErrors {
        fn from(value: FeeTooLarge) -> Self {
            Self::FeeTooLarge(value)
        }
    }
    impl ::core::convert::From<IncorrectAmounts> for MockDutchOrderReactorErrors {
        fn from(value: IncorrectAmounts) -> Self {
            Self::IncorrectAmounts(value)
        }
    }
    impl ::core::convert::From<InputAndOutputDecay> for MockDutchOrderReactorErrors {
        fn from(value: InputAndOutputDecay) -> Self {
            Self::InputAndOutputDecay(value)
        }
    }
    impl ::core::convert::From<InputAndOutputFees> for MockDutchOrderReactorErrors {
        fn from(value: InputAndOutputFees) -> Self {
            Self::InputAndOutputFees(value)
        }
    }
    impl ::core::convert::From<InvalidFeeToken> for MockDutchOrderReactorErrors {
        fn from(value: InvalidFeeToken) -> Self {
            Self::InvalidFeeToken(value)
        }
    }
    impl ::core::convert::From<InvalidReactor> for MockDutchOrderReactorErrors {
        fn from(value: InvalidReactor) -> Self {
            Self::InvalidReactor(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for MockDutchOrderReactorErrors {
        fn from(value: NativeTransferFailed) -> Self {
            Self::NativeTransferFailed(value)
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
    pub enum MockDutchOrderReactorEvents {
        FillFilter(FillFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProtocolFeeControllerSetFilter(ProtocolFeeControllerSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockDutchOrderReactorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FillFilter::decode_log(log) {
                return Ok(MockDutchOrderReactorEvents::FillFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    MockDutchOrderReactorEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = ProtocolFeeControllerSetFilter::decode_log(log) {
                return Ok(
                    MockDutchOrderReactorEvents::ProtocolFeeControllerSetFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockDutchOrderReactorEvents {
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
    impl ::core::convert::From<FillFilter> for MockDutchOrderReactorEvents {
        fn from(value: FillFilter) -> Self {
            Self::FillFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for MockDutchOrderReactorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeControllerSetFilter>
    for MockDutchOrderReactorEvents {
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
    ///Container type for all input parameters for the `resolveOrder` function with signature `resolveOrder((bytes,bytes))` and selector `0x9bb32265`
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
    #[ethcall(name = "resolveOrder", abi = "resolveOrder((bytes,bytes))")]
    pub struct ResolveOrderCall {
        pub order: SignedOrder,
    }
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
    pub enum MockDutchOrderReactorCalls {
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        ExecuteBatchWithCallback(ExecuteBatchWithCallbackCall),
        ExecuteWithCallback(ExecuteWithCallbackCall),
        FeeController(FeeControllerCall),
        Owner(OwnerCall),
        Permit2(Permit2Call),
        ResolveOrder(ResolveOrderCall),
        SetProtocolFeeController(SetProtocolFeeControllerCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockDutchOrderReactorCalls {
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
            if let Ok(decoded) = <ResolveOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResolveOrder(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MockDutchOrderReactorCalls {
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
                Self::ResolveOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProtocolFeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockDutchOrderReactorCalls {
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
                Self::ResolveOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for MockDutchOrderReactorCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall> for MockDutchOrderReactorCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchWithCallbackCall>
    for MockDutchOrderReactorCalls {
        fn from(value: ExecuteBatchWithCallbackCall) -> Self {
            Self::ExecuteBatchWithCallback(value)
        }
    }
    impl ::core::convert::From<ExecuteWithCallbackCall> for MockDutchOrderReactorCalls {
        fn from(value: ExecuteWithCallbackCall) -> Self {
            Self::ExecuteWithCallback(value)
        }
    }
    impl ::core::convert::From<FeeControllerCall> for MockDutchOrderReactorCalls {
        fn from(value: FeeControllerCall) -> Self {
            Self::FeeController(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MockDutchOrderReactorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<Permit2Call> for MockDutchOrderReactorCalls {
        fn from(value: Permit2Call) -> Self {
            Self::Permit2(value)
        }
    }
    impl ::core::convert::From<ResolveOrderCall> for MockDutchOrderReactorCalls {
        fn from(value: ResolveOrderCall) -> Self {
            Self::ResolveOrder(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeControllerCall>
    for MockDutchOrderReactorCalls {
        fn from(value: SetProtocolFeeControllerCall) -> Self {
            Self::SetProtocolFeeController(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MockDutchOrderReactorCalls {
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
    ///Container type for all return fields from the `resolveOrder` function with signature `resolveOrder((bytes,bytes))` and selector `0x9bb32265`
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
    pub struct ResolveOrderReturn {
        pub resolved_order: ResolvedOrder,
    }
}
