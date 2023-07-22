pub use i_signature_transfer::*;
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
pub mod i_signature_transfer {
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
            ]),
            events: ::core::convert::From::from([
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
    pub static ISIGNATURETRANSFER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct ISignatureTransfer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISignatureTransfer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISignatureTransfer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISignatureTransfer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISignatureTransfer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ISignatureTransfer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISignatureTransfer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISIGNATURETRANSFER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
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
            UnorderedNonceInvalidationFilter,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ISignatureTransfer<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
        Hash,
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
        Hash,
    )]
    #[etherror(name = "LengthMismatch", abi = "LengthMismatch()")]
    pub struct LengthMismatch;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISignatureTransferErrors {
        InvalidAmount(InvalidAmount),
        LengthMismatch(LengthMismatch),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ISignatureTransferErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAmount(decoded));
            }
            if let Ok(decoded) = <LengthMismatch as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LengthMismatch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISignatureTransferErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LengthMismatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ISignatureTransferErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <InvalidAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <LengthMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ISignatureTransferErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::LengthMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ISignatureTransferErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidAmount> for ISignatureTransferErrors {
        fn from(value: InvalidAmount) -> Self {
            Self::InvalidAmount(value)
        }
    }
    impl ::core::convert::From<LengthMismatch> for ISignatureTransferErrors {
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
        Hash,
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
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `invalidateUnorderedNonces` function with signature `invalidateUnorderedNonces(uint256,uint256)` and selector `0x3ff9dcb1`
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
        name = "invalidateUnorderedNonces",
        abi = "invalidateUnorderedNonces(uint256,uint256)"
    )]
    pub struct InvalidateUnorderedNoncesCall {
        pub word_pos: ::ethers::core::types::U256,
        pub mask: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "nonceBitmap", abi = "nonceBitmap(address,uint256)")]
    pub struct NonceBitmapCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `permitTransferFrom` function with signature `permitTransferFrom(((address,uint256),uint256,uint256),(address,uint256),address,bytes)` and selector `0x30f28b7a`
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "permitWitnessTransferFrom",
        abi = "permitWitnessTransferFrom(((address,uint256)[],uint256,uint256),(address,uint256)[],address,bytes32,string,bytes)"
    )]
    pub struct PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall
    {
        pub permit: PermitBatchTransferFrom,
        pub transfer_details: ::std::vec::Vec<SignatureTransferDetails>,
        pub owner: ::ethers::core::types::Address,
        pub witness: [u8; 32],
        pub witness_type_string: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISignatureTransferCalls {
        DomainSeparator(DomainSeparatorCall),
        InvalidateUnorderedNonces(InvalidateUnorderedNoncesCall),
        NonceBitmap(NonceBitmapCall),
        PermitTransferFrom(PermitTransferFromCall),
        PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignature(
            PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall,
        ),
        PermitWitnessTransferFrom(PermitWitnessTransferFromCall),
        PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignature(
            PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall,
        ),
    }
    impl ::ethers::core::abi::AbiDecode for ISignatureTransferCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <InvalidateUnorderedNoncesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidateUnorderedNonces(decoded));
            }
            if let Ok(decoded) = <NonceBitmapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonceBitmap(decoded));
            }
            if let Ok(decoded) =
                <PermitTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
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
            if let Ok(decoded) =
                <PermitWitnessTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISignatureTransferCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidateUnorderedNonces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NonceBitmap(element) => {
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
            }
        }
    }
    impl ::core::fmt::Display for ISignatureTransferCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidateUnorderedNonces(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NonceBitmap(element) => ::core::fmt::Display::fmt(element, f),
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
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for ISignatureTransferCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<InvalidateUnorderedNoncesCall> for ISignatureTransferCalls {
        fn from(value: InvalidateUnorderedNoncesCall) -> Self {
            Self::InvalidateUnorderedNonces(value)
        }
    }
    impl ::core::convert::From<NonceBitmapCall> for ISignatureTransferCalls {
        fn from(value: NonceBitmapCall) -> Self {
            Self::NonceBitmap(value)
        }
    }
    impl ::core::convert::From<PermitTransferFromCall> for ISignatureTransferCalls {
        fn from(value: PermitTransferFromCall) -> Self {
            Self::PermitTransferFrom(value)
        }
    }
    impl
        ::core::convert::From<
            PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall,
        > for ISignatureTransferCalls
    {
        fn from(
            value: PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignatureCall,
        ) -> Self {
            Self::PermitTransferFromWithPermitAndTransferDetailsAndOwnerAndSignature(value)
        }
    }
    impl ::core::convert::From<PermitWitnessTransferFromCall> for ISignatureTransferCalls {
        fn from(value: PermitWitnessTransferFromCall) -> Self {
            Self::PermitWitnessTransferFrom(value)
        }
    }
    impl ::core::convert::From<
        PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall,
    > for ISignatureTransferCalls {
        fn from(
            value: PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignatureCall,
        ) -> Self {
            Self::PermitWitnessTransferFromWithPermitAndTransferDetailsAndOwnerAndWitnessAndWitnessTypeStringAndSignature(
                value,
            )
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
        Hash,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `nonceBitmap` function with signature `nonceBitmap(address,uint256)` and selector `0x4fe02b44`
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
    pub struct NonceBitmapReturn(pub ::ethers::core::types::U256);
}
