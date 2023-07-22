pub use i_permit_2::*;
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
pub mod i_permit_2 {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
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
                    ::std::borrow::ToOwned::to_owned("invalidateNonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("invalidateNonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint48"),
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
                    ::std::borrow::ToOwned::to_owned("invalidateUnorderedNonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "invalidateUnorderedNonces",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("wordPos"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mask"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("lockdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lockdown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approvals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAllowanceTransfer.TokenSpenderPair[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("nonceBitmap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonceBitmap"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitBatch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAllowanceTransfer.PermitBatch",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitSingle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAllowanceTransfer.PermitSingle",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                    ::std::borrow::ToOwned::to_owned("permitTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permitTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.PermitTransferFrom",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferDetails"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.SignatureTransferDetails",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permitTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.PermitBatchTransferFrom",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferDetails"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.SignatureTransferDetails[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                    ::std::borrow::ToOwned::to_owned("permitWitnessTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "permitWitnessTransferFrom",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.PermitTransferFrom",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferDetails"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.SignatureTransferDetails",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("witness"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("witnessTypeString"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "permitWitnessTransferFrom",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.PermitBatchTransferFrom",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferDetails"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureTransfer.SignatureTransferDetails[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("witness"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("witnessTypeString"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
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
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("transferDetails"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAllowanceTransfer.AllowanceTransferDetails[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("expiration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Lockdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Lockdown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NonceInvalidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NonceInvalidation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("expiration"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(48usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnorderedNonceInvalidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnorderedNonceInvalidation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("word"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mask"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AllowanceExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AllowanceExpired"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExcessiveInvalidation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ExcessiveInvalidation",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientAllowance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LengthMismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("LengthMismatch"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IPERMIT2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IPermit2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPermit2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPermit2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPermit2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPermit2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IPermit2)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPermit2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IPERMIT2_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0x927da105) function
        pub fn allowance(
            &self,
            user: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, u64, u64),
        > {
            self.0
                .method_hash([146, 125, 161, 5], (user, token, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x87517c45) function
        pub fn approve(
            &self,
            token: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            expiration: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 81, 124, 69], (token, spender, amount, expiration))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invalidateNonces` (0x65d9723c) function
        pub fn invalidate_nonces(
            &self,
            token: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            new_nonce: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 217, 114, 60], (token, spender, new_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invalidateUnorderedNonces` (0x3ff9dcb1) function
        pub fn invalidate_unordered_nonces(
            &self,
            word_pos: ::ethers::core::types::U256,
            mask: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 249, 220, 177], (word_pos, mask))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockdown` (0xcc53287f) function
        pub fn lockdown(
            &self,
            approvals: ::std::vec::Vec<TokenSpenderPair>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 83, 40, 127], approvals)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonceBitmap` (0x4fe02b44) function
        pub fn nonce_bitmap(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 224, 43, 68], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0x2a2d80d1) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            permit_batch: PermitBatch,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 45, 128, 209], (owner, permit_batch, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0x2b67b570) function
        pub fn permit_with_owner_and_permit_single(
            &self,
            owner: ::ethers::core::types::Address,
            permit_single: PermitSingle,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 103, 181, 112], (owner, permit_single, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permitTransferFrom` (0x30f28b7a) function
        pub fn permit_transfer_from(
            &self,
            permit: PermitBatchTransferFrom,
            transfer_details: SignatureTransferDetails,
            owner: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [48, 242, 139, 122],
                    (permit, transfer_details, owner, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permitTransferFrom` (0xedd9444b) function
        pub fn permit_transfer_from_with_permit_and_transfer_details_and_owner_and_signature(
            &self,
            permit: PermitBatchTransferFrom,
            transfer_details: ::std::vec::Vec<SignatureTransferDetails>,
            owner: ::ethers::core::types::Address,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [237, 217, 68, 75],
                    (permit, transfer_details, owner, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permitWitnessTransferFrom` (0x137c29fe) function
        pub fn permit_witness_transfer_from(
            &self,
            permit: PermitBatchTransferFrom,
            transfer_details: SignatureTransferDetails,
            owner: ::ethers::core::types::Address,
            witness: [u8; 32],
            witness_type_string: ::std::string::String,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [19, 124, 41, 254],
                    (
                        permit,
                        transfer_details,
                        owner,
                        witness,
                        witness_type_string,
                        signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permitWitnessTransferFrom` (0xfe8ec1a7) function
        pub fn permit_witness_transfer_from_with_permit_and_transfer_details_and_owner_and_witness_and_witness_type_string_and_signature(
            &self,
            permit: PermitBatchTransferFrom,
            transfer_details: ::std::vec::Vec<SignatureTransferDetails>,
            owner: ::ethers::core::types::Address,
            witness: [u8; 32],
            witness_type_string: ::std::string::String,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [254, 142, 193, 167],
                    (
                        permit,
                        transfer_details,
                        owner,
                        witness,
                        witness_type_string,
                        signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x0d58b1db) function
        pub fn transfer_from(
            &self,
            transfer_details: ::std::vec::Vec<AllowanceTransferDetails>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 88, 177, 219], transfer_details)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x36c78516) function
        pub fn transfer_from_with_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 199, 133, 22], (from, to, amount, token))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Lockdown` event
        pub fn lockdown_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LockdownFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NonceInvalidation` event
        pub fn nonce_invalidation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NonceInvalidationFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Permit` event
        pub fn permit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PermitFilter> {
            self.0.event()
        }
        ///Gets the contract's `UnorderedNonceInvalidation` event
        pub fn unordered_nonce_invalidation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnorderedNonceInvalidationFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IPermit2Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IPermit2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AllowanceExpired` with signature `AllowanceExpired(uint256)` and selector `0xd81b2f2e`
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
    #[etherror(name = "AllowanceExpired", abi = "AllowanceExpired(uint256)")]
    pub struct AllowanceExpired {
        pub deadline: ::ethers::core::types::U256,
    }
    ///Custom Error type `ExcessiveInvalidation` with signature `ExcessiveInvalidation()` and selector `0x24d35a26`
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
    #[etherror(name = "ExcessiveInvalidation", abi = "ExcessiveInvalidation()")]
    pub struct ExcessiveInvalidation;
    ///Custom Error type `InsufficientAllowance` with signature `InsufficientAllowance(uint256)` and selector `0xf96fb071`
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
    #[etherror(name = "InsufficientAllowance", abi = "InsufficientAllowance(uint256)")]
    pub struct InsufficientAllowance {
        pub amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidAmount` with signature `InvalidAmount(uint256)` and selector `0x3728b83d`
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
    #[etherror(name = "InvalidAmount", abi = "InvalidAmount(uint256)")]
    pub struct InvalidAmount {
        pub max_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `LengthMismatch` with signature `LengthMismatch()` and selector `0xff633a38`
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
    #[etherror(name = "LengthMismatch", abi = "LengthMismatch()")]
    pub struct LengthMismatch;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPermit2Errors {
        AllowanceExpired(AllowanceExpired),
        ExcessiveInvalidation(ExcessiveInvalidation),
        InsufficientAllowance(InsufficientAllowance),
        InvalidAmount(InvalidAmount),
        LengthMismatch(LengthMismatch),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IPermit2Errors {
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
                = <AllowanceExpired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllowanceExpired(decoded));
            }
            if let Ok(decoded)
                = <ExcessiveInvalidation as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExcessiveInvalidation(decoded));
            }
            if let Ok(decoded)
                = <InsufficientAllowance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InsufficientAllowance(decoded));
            }
            if let Ok(decoded)
                = <InvalidAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAmount(decoded));
            }
            if let Ok(decoded)
                = <LengthMismatch as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LengthMismatch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPermit2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AllowanceExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcessiveInvalidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LengthMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for IPermit2Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AllowanceExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExcessiveInvalidation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientAllowance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LengthMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IPermit2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowanceExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcessiveInvalidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::LengthMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IPermit2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AllowanceExpired> for IPermit2Errors {
        fn from(value: AllowanceExpired) -> Self {
            Self::AllowanceExpired(value)
        }
    }
    impl ::core::convert::From<ExcessiveInvalidation> for IPermit2Errors {
        fn from(value: ExcessiveInvalidation) -> Self {
            Self::ExcessiveInvalidation(value)
        }
    }
    impl ::core::convert::From<InsufficientAllowance> for IPermit2Errors {
        fn from(value: InsufficientAllowance) -> Self {
            Self::InsufficientAllowance(value)
        }
    }
    impl ::core::convert::From<InvalidAmount> for IPermit2Errors {
        fn from(value: InvalidAmount) -> Self {
            Self::InvalidAmount(value)
        }
    }
    impl ::core::convert::From<LengthMismatch> for IPermit2Errors {
        fn from(value: LengthMismatch) -> Self {
            Self::LengthMismatch(value)
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
        name = "Approval",
        abi = "Approval(address,address,address,uint160,uint48)"
    )]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub expiration: u64,
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
    #[ethevent(name = "Lockdown", abi = "Lockdown(address,address,address)")]
    pub struct LockdownFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
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
        name = "NonceInvalidation",
        abi = "NonceInvalidation(address,address,address,uint48,uint48)"
    )]
    pub struct NonceInvalidationFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub new_nonce: u64,
        pub old_nonce: u64,
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
        name = "Permit",
        abi = "Permit(address,address,address,uint160,uint48,uint48)"
    )]
    pub struct PermitFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub expiration: u64,
        pub nonce: u64,
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
        name = "UnorderedNonceInvalidation",
        abi = "UnorderedNonceInvalidation(address,uint256,uint256)"
    )]
    pub struct UnorderedNonceInvalidationFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub word: ::ethers::core::types::U256,
        pub mask: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPermit2Events {
        ApprovalFilter(ApprovalFilter),
        LockdownFilter(LockdownFilter),
        NonceInvalidationFilter(NonceInvalidationFilter),
        PermitFilter(PermitFilter),
        UnorderedNonceInvalidationFilter(UnorderedNonceInvalidationFilter),
    }
    impl ::ethers::contract::EthLogDecode for IPermit2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IPermit2Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = LockdownFilter::decode_log(log) {
                return Ok(IPermit2Events::LockdownFilter(decoded));
            }
            if let Ok(decoded) = NonceInvalidationFilter::decode_log(log) {
                return Ok(IPermit2Events::NonceInvalidationFilter(decoded));
            }
            if let Ok(decoded) = PermitFilter::decode_log(log) {
                return Ok(IPermit2Events::PermitFilter(decoded));
            }
            if let Ok(decoded) = UnorderedNonceInvalidationFilter::decode_log(log) {
                return Ok(IPermit2Events::UnorderedNonceInvalidationFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IPermit2Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockdownFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonceInvalidationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnorderedNonceInvalidationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for IPermit2Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<LockdownFilter> for IPermit2Events {
        fn from(value: LockdownFilter) -> Self {
            Self::LockdownFilter(value)
        }
    }
    impl ::core::convert::From<NonceInvalidationFilter> for IPermit2Events {
        fn from(value: NonceInvalidationFilter) -> Self {
            Self::NonceInvalidationFilter(value)
        }
    }
    impl ::core::convert::From<PermitFilter> for IPermit2Events {
        fn from(value: PermitFilter) -> Self {
            Self::PermitFilter(value)
        }
    }
    impl ::core::convert::From<UnorderedNonceInvalidationFilter> for IPermit2Events {
        fn from(value: UnorderedNonceInvalidationFilter) -> Self {
            Self::UnorderedNonceInvalidationFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address,address)` and selector `0x927da105`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address,address)")]
    pub struct AllowanceCall {
        pub user: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,address,uint160,uint48)` and selector `0x87517c45`
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
    #[ethcall(name = "approve", abi = "approve(address,address,uint160,uint48)")]
    pub struct ApproveCall {
        pub token: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub expiration: u64,
    }
    ///Container type for all input parameters for the `invalidateNonces` function with signature `invalidateNonces(address,address,uint48)` and selector `0x65d9723c`
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
        name = "invalidateNonces",
        abi = "invalidateNonces(address,address,uint48)"
    )]
    pub struct InvalidateNoncesCall {
        pub token: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub new_nonce: u64,
    }
    ///Container type for all input parameters for the `invalidateUnorderedNonces` function with signature `invalidateUnorderedNonces(uint256,uint256)` and selector `0x3ff9dcb1`
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
        name = "invalidateUnorderedNonces",
        abi = "invalidateUnorderedNonces(uint256,uint256)"
    )]
    pub struct InvalidateUnorderedNoncesCall {
        pub word_pos: ::ethers::core::types::U256,
        pub mask: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lockdown` function with signature `lockdown((address,address)[])` and selector `0xcc53287f`
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
    #[ethcall(name = "lockdown", abi = "lockdown((address,address)[])")]
    pub struct LockdownCall {
        pub approvals: ::std::vec::Vec<TokenSpenderPair>,
    }
    ///Container type for all input parameters for the `nonceBitmap` function with signature `nonceBitmap(address,uint256)` and selector `0x4fe02b44`
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
    #[ethcall(name = "nonceBitmap", abi = "nonceBitmap(address,uint256)")]
    pub struct NonceBitmapCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `permit` function with signature `permit(address,((address,uint160,uint48,uint48)[],address,uint256),bytes)` and selector `0x2a2d80d1`
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
        name = "permit",
        abi = "permit(address,((address,uint160,uint48,uint48)[],address,uint256),bytes)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub permit_batch: PermitBatch,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `permit` function with signature `permit(address,((address,uint160,uint48,uint48),address,uint256),bytes)` and selector `0x2b67b570`
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
        name = "permit",
        abi = "permit(address,((address,uint160,uint48,uint48),address,uint256),bytes)"
    )]
    pub struct PermitWithOwnerAndPermitSingleCall {
        pub owner: ::ethers::core::types::Address,
        pub permit_single: PermitSingle,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `permitTransferFrom` function with signature `permitTransferFrom(((address,uint256),uint256,uint256),(address,uint256),address,bytes)` and selector `0x30f28b7a`
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
        name = "permitTransferFrom",
        abi = "permitTransferFrom(((address,uint256),uint256,uint256),(address,uint256),address,bytes)"
    )]
    pub struct PermitTransferFromCall {
        pub permit: PermitBatchTransferFrom,
        pub transfer_details: SignatureTransferDetails,
        pub owner: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `permitTransferFrom` function with signature `permitTransferFrom(((address,uint256)[],uint256,uint256),(address,uint256)[],address,bytes)` and selector `0xedd9444b`
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
        name = "permitTransferFrom",
        abi = "permitTransferFrom(((address,uint256)[],uint256,uint256),(address,uint256)[],address,bytes)"
    )]
    pub struct PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall {
        pub permit: PermitBatchTransferFrom,
        pub transfer_details: ::std::vec::Vec<SignatureTransferDetails>,
        pub owner: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `permitWitnessTransferFrom` function with signature `permitWitnessTransferFrom(((address,uint256),uint256,uint256),(address,uint256),address,bytes32,string,bytes)` and selector `0x137c29fe`
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
        name = "permitWitnessTransferFrom",
        abi = "permitWitnessTransferFrom(((address,uint256),uint256,uint256),(address,uint256),address,bytes32,string,bytes)"
    )]
    pub struct PermitWitnessTransferFromCall {
        pub permit: PermitBatchTransferFrom,
        pub transfer_details: SignatureTransferDetails,
        pub owner: ::ethers::core::types::Address,
        pub witness: [u8; 32],
        pub witness_type_string: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `permitWitnessTransferFrom` function with signature `permitWitnessTransferFrom(((address,uint256)[],uint256,uint256),(address,uint256)[],address,bytes32,string,bytes)` and selector `0xfe8ec1a7`
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
        name = "permitWitnessTransferFrom",
        abi = "permitWitnessTransferFrom(((address,uint256)[],uint256,uint256),(address,uint256)[],address,bytes32,string,bytes)"
    )]
    pub struct PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall {
        pub permit: PermitBatchTransferFrom,
        pub transfer_details: ::std::vec::Vec<SignatureTransferDetails>,
        pub owner: ::ethers::core::types::Address,
        pub witness: [u8; 32],
        pub witness_type_string: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom((address,address,uint160,address)[])` and selector `0x0d58b1db`
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
        name = "transferFrom",
        abi = "transferFrom((address,address,uint160,address)[])"
    )]
    pub struct TransferFromCall {
        pub transfer_details: ::std::vec::Vec<AllowanceTransferDetails>,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint160,address)` and selector `0x36c78516`
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
        name = "transferFrom",
        abi = "transferFrom(address,address,uint160,address)"
    )]
    pub struct TransferFromWithFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IPermit2Calls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        InvalidateNonces(InvalidateNoncesCall),
        InvalidateUnorderedNonces(InvalidateUnorderedNoncesCall),
        Lockdown(LockdownCall),
        NonceBitmap(NonceBitmapCall),
        Permit(PermitCall),
        PermitWithOwnerAndPermitSingle(PermitWithOwnerAndPermitSingleCall),
        PermitTransferFrom(PermitTransferFromCall),
        PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignature(
            PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall,
        ),
        PermitWitnessTransferFrom(PermitWitnessTransferFromCall),
        PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignature(
            PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall,
        ),
        TransferFrom(TransferFromCall),
        TransferFromWithFrom(TransferFromWithFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for IPermit2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <InvalidateNoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidateNonces(decoded));
            }
            if let Ok(decoded)
                = <InvalidateUnorderedNoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidateUnorderedNonces(decoded));
            }
            if let Ok(decoded)
                = <LockdownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Lockdown(decoded));
            }
            if let Ok(decoded)
                = <NonceBitmapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonceBitmap(decoded));
            }
            if let Ok(decoded)
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded)
                = <PermitWithOwnerAndPermitSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PermitWithOwnerAndPermitSingle(decoded));
            }
            if let Ok(decoded)
                = <PermitTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PermitTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignature(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = <PermitWitnessTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PermitWitnessTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignature(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TransferFromWithFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferFromWithFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IPermit2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidateNonces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidateUnorderedNonces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Lockdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonceBitmap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PermitWithOwnerAndPermitSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermitTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignature(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PermitWitnessTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignature(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromWithFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IPermit2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidateNonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidateUnorderedNonces(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Lockdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonceBitmap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermitWithOwnerAndPermitSingle(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermitTransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignature(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::PermitWitnessTransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignature(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFromWithFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for IPermit2Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for IPermit2Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for IPermit2Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<InvalidateNoncesCall> for IPermit2Calls {
        fn from(value: InvalidateNoncesCall) -> Self {
            Self::InvalidateNonces(value)
        }
    }
    impl ::core::convert::From<InvalidateUnorderedNoncesCall> for IPermit2Calls {
        fn from(value: InvalidateUnorderedNoncesCall) -> Self {
            Self::InvalidateUnorderedNonces(value)
        }
    }
    impl ::core::convert::From<LockdownCall> for IPermit2Calls {
        fn from(value: LockdownCall) -> Self {
            Self::Lockdown(value)
        }
    }
    impl ::core::convert::From<NonceBitmapCall> for IPermit2Calls {
        fn from(value: NonceBitmapCall) -> Self {
            Self::NonceBitmap(value)
        }
    }
    impl ::core::convert::From<PermitCall> for IPermit2Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PermitWithOwnerAndPermitSingleCall> for IPermit2Calls {
        fn from(value: PermitWithOwnerAndPermitSingleCall) -> Self {
            Self::PermitWithOwnerAndPermitSingle(value)
        }
    }
    impl ::core::convert::From<PermitTransferFromCall> for IPermit2Calls {
        fn from(value: PermitTransferFromCall) -> Self {
            Self::PermitTransferFrom(value)
        }
    }
    impl ::core::convert::From<
        PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall,
    > for IPermit2Calls {
        fn from(
            value: PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall,
        ) -> Self {
            Self::PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignature(
                value,
            )
        }
    }
    impl ::core::convert::From<PermitWitnessTransferFromCall> for IPermit2Calls {
        fn from(value: PermitWitnessTransferFromCall) -> Self {
            Self::PermitWitnessTransferFrom(value)
        }
    }
    impl ::core::convert::From<
        PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall,
    > for IPermit2Calls {
        fn from(
            value: PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall,
        ) -> Self {
            Self::PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignature(
                value,
            )
        }
    }
    impl ::core::convert::From<TransferFromCall> for IPermit2Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferFromWithFromCall> for IPermit2Calls {
        fn from(value: TransferFromWithFromCall) -> Self {
            Self::TransferFromWithFrom(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address,address)` and selector `0x927da105`
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
    pub struct AllowanceReturn {
        pub amount: ::ethers::core::types::U256,
        pub expiration: u64,
        pub nonce: u64,
    }
    ///Container type for all return fields from the `nonceBitmap` function with signature `nonceBitmap(address,uint256)` and selector `0x4fe02b44`
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
    pub struct NonceBitmapReturn(pub ::ethers::core::types::U256);
}
