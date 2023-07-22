pub use mock_protocol_fees::*;
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
pub mod mock_protocol_fees {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_protocolFeeOwner"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("takeFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("takeFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("order"),
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
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
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
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ResolvedOrder"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKPROTOCOLFEES_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x10\xBB8\x03\x80a\x10\xBB\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x81V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x83\x92\x83\x92\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3PPPa\0\xB1V[`\0` \x82\x84\x03\x12\x15a\0\x93W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xAAW`\0\x80\xFD[\x93\x92PPPV[a\x0F\xFB\x80a\0\xC0`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80co\x12\xF4B\x11a\0PW\x80co\x12\xF4B\x14a\0\xCBW\x80c\x8D\xA5\xCB[\x14a\0\xEBW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x0BW`\0\x80\xFD[\x80c-w\x13\x89\x14a\0lW\x80ci\x99\xB3w\x14a\0\x81W[`\0\x80\xFD[a\0\x7Fa\0z6`\x04a\x08rV[a\x01\x1EV[\0[`\x01Ta\0\xA1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDEa\0\xD96`\x04a\x0B\xBCV[a\x02*V[`@Qa\0\xC2\x91\x90a\r\\V[`\0Ta\0\xA1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\0\x7Fa\x01\x196`\x04a\x08rV[a\x02\xA6V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x01\xA4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x02\xA2\x82a\x03\x97V[P\x90V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03'W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x9BV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xB7WPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x04\x0E\x90\x85\x90`\x04\x01a\r\\V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04q\x91\x90\x81\x01\x90a\x0E\x86V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x04\x89\x82\x84a\x0FVV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xA1Wa\x04\xA1a\x08\x96V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\nW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x04\xBFW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x05[W\x85`@\x01Q\x81\x81Q\x81\x10a\x05.Wa\x05.a\x0F\x96V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x05HWa\x05Ha\x0F\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\x10V[P`\0[\x82\x81\x10\x15a\x08\x02W`\0\x85\x82\x81Q\x81\x10a\x05{Wa\x05{a\x0F\x96V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x069W\x86\x81\x81Q\x81\x10a\x05\xA2Wa\x05\xA2a\x0F\x96V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x061W\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x9BV[`\x01\x01a\x05\x88V[P`\0\x80[\x86\x81\x10\x15a\x06\xBEW`\0\x89`@\x01Q\x82\x81Q\x81\x10a\x06^Wa\x06^a\x0F\x96V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x06\xB5W` \x81\x01Qa\x06\xB2\x90\x84a\x0FVV[\x92P[P`\x01\x01a\x06>V[P\x81Q` \x89\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x06\xFBW` \x80\x89\x01Q\x01Qa\x06\xF8\x90\x82a\x0FVV[\x90P[\x80`\0\x03a\x07PW\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x9BV[a\x07^\x81`\x05a'\x10a\x08\x11V[\x82` \x01Q\x11\x15a\x07\xD1W\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x01\x9BV[\x81\x84a\x07\xDD\x85\x89a\x0FVV[\x81Q\x81\x10a\x07\xEDWa\x07\xEDa\x0F\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05_V[P`@\x90\x94\x01\x93\x90\x93RPPPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x08FW`\0\x80\xFD[P\x91\x02\x04\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08oW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x08\x84W`\0\x80\xFD[\x815a\x08\x8F\x81a\x08MV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\xE8Wa\x08\xE8a\x08\x96V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\xE8Wa\x08\xE8a\x08\x96V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\tXWa\tXa\x08\x96V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\tqW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x8BWa\t\x8Ba\x08\x96V[a\t\xBC` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\t\x11V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\t\xD1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\n\0W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\n$Wa\n$a\x08\x96V[\x81`@R\x82\x93P\x845\x91Pa\n8\x82a\x08MV[\x90\x82R` \x84\x015\x90a\nJ\x82a\x08MV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\nt\x82a\x08MV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\n\x8EW`\0\x80\xFD[Pa\n\x9B\x85\x82\x86\x01a\t`V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\n\xBAW`\0\x80\xFD[a\n\xC2a\x08\xC5V[\x90P\x815a\n\xCF\x81a\x08MV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\x07Wa\x0B\x07a\x08\x96V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0B\"W`\0\x80\xFD[\x815` a\x0B7a\x0B2\x83a\n\xEDV[a\t\x11V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x0BVW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x0B\xAFW\x81\x81\x8A\x03\x12\x15a\x0BrW`\0\x80\x81\xFD[a\x0Bza\x08\xC5V[\x815a\x0B\x85\x81a\x08MV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x0B\x9E\x81a\x08MV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x0BZV[P\x90\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xCEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xE6W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x0B\xFAW`\0\x80\xFD[a\x0C\x02a\x08\xEEV[\x825\x82\x81\x11\x15a\x0C\x11W`\0\x80\xFD[a\x0C\x1D\x87\x82\x86\x01a\t\xEEV[\x82RPa\x0C-\x86` \x85\x01a\n\xA8V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x0CDW`\0\x80\xFD[a\x0CP\x87\x82\x86\x01a\x0B\x11V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x0ChW`\0\x80\xFD[a\x0Ct\x87\x82\x86\x01a\t`V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0C\xB4W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0C\x98V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\rQW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\r\x06V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\r\xD8a\x01\xC0\x84\x01\x82a\x0C\x8EV[\x90P` \x84\x01Qa\x0E\x16`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x0EQ\x83\x83a\x0C\xF2V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x0Eo\x82\x82a\x0C\x8EV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x0E\x99W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xB0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0E\xC1W`\0\x80\xFD[\x80Qa\x0E\xCFa\x0B2\x82a\n\xEDV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a\x0E\xEEW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x0FJW\x80\x85\x8A\x03\x12\x15a\x0F\x0BW`\0\x80\x81\xFD[a\x0F\x13a\x08\xC5V[\x85Qa\x0F\x1E\x81a\x08MV[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa\x0F7\x81a\x08MV[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a\x0E\xF3V[P\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0F\x90W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA5\x02\xDA\xE0x\x0C\xB2\xAA\xA3%\x9D\xB6\xC5\xDF?\xE9\x03\xCD3\x9E#\xDAW#{\x17\xE6\xD4\xF2\xDC\x11\xF2dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKPROTOCOLFEES_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0gW`\x005`\xE0\x1C\x80co\x12\xF4B\x11a\0PW\x80co\x12\xF4B\x14a\0\xCBW\x80c\x8D\xA5\xCB[\x14a\0\xEBW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x0BW`\0\x80\xFD[\x80c-w\x13\x89\x14a\0lW\x80ci\x99\xB3w\x14a\0\x81W[`\0\x80\xFD[a\0\x7Fa\0z6`\x04a\x08rV[a\x01\x1EV[\0[`\x01Ta\0\xA1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDEa\0\xD96`\x04a\x0B\xBCV[a\x02*V[`@Qa\0\xC2\x91\x90a\r\\V[`\0Ta\0\xA1\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\0\x7Fa\x01\x196`\x04a\x08rV[a\x02\xA6V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x01\xA4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@\x80Q\x91\x90\x92\x16\x80\x82R` \x82\x01\x93\x90\x93R\x7F\xB9\x04\xAE\x95)\xE3s\xE4\x8B\xC8-\xF42l\xCE\xAF\x1BLG+\xAB\xF3\x7F[}\xECF\xFE\xCCkS\xE0\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x02\xA2\x82a\x03\x97V[P\x90V[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x03'W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x9BV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[`\x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x03\xB7WPV[`\x01T`@Q\x7F\x8A\xA6\xCF\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90c\x8A\xA6\xCF\x03\x90a\x04\x0E\x90\x85\x90`\x04\x01a\r\\V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04q\x91\x90\x81\x01\x90a\x0E\x86V[`@\x83\x01QQ\x81Q\x91\x92P\x90`\0a\x04\x89\x82\x84a\x0FVV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xA1Wa\x04\xA1a\x08\x96V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\nW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x04\xBFW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x05[W\x85`@\x01Q\x81\x81Q\x81\x10a\x05.Wa\x05.a\x0F\x96V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x05HWa\x05Ha\x0F\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\x10V[P`\0[\x82\x81\x10\x15a\x08\x02W`\0\x85\x82\x81Q\x81\x10a\x05{Wa\x05{a\x0F\x96V[` \x02` \x01\x01Q\x90P`\0[\x82\x81\x10\x15a\x069W\x86\x81\x81Q\x81\x10a\x05\xA2Wa\x05\xA2a\x0F\x96V[` \x02` \x01\x01Q`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x061W\x81Q`@Q\x7F\xFF\xF0\x83\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x9BV[`\x01\x01a\x05\x88V[P`\0\x80[\x86\x81\x10\x15a\x06\xBEW`\0\x89`@\x01Q\x82\x81Q\x81\x10a\x06^Wa\x06^a\x0F\x96V[` \x02` \x01\x01Q\x90P\x83`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x06\xB5W` \x81\x01Qa\x06\xB2\x90\x84a\x0FVV[\x92P[P`\x01\x01a\x06>V[P\x81Q` \x89\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x03a\x06\xFBW` \x80\x89\x01Q\x01Qa\x06\xF8\x90\x82a\x0FVV[\x90P[\x80`\0\x03a\x07PW\x81Q`@Q\x7F\xED\xDF\x07\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x9BV[a\x07^\x81`\x05a'\x10a\x08\x11V[\x82` \x01Q\x11\x15a\x07\xD1W\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Q\x7F\x82\xE7VV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x92\x90\x92R\x91\x90\x91\x16`D\x82\x01R`d\x01a\x01\x9BV[\x81\x84a\x07\xDD\x85\x89a\x0FVV[\x81Q\x81\x10a\x07\xEDWa\x07\xEDa\x0F\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x05_V[P`@\x90\x94\x01\x93\x90\x93RPPPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x08FW`\0\x80\xFD[P\x91\x02\x04\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08oW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x08\x84W`\0\x80\xFD[\x815a\x08\x8F\x81a\x08MV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\xE8Wa\x08\xE8a\x08\x96V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\xE8Wa\x08\xE8a\x08\x96V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\tXWa\tXa\x08\x96V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\tqW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x8BWa\t\x8Ba\x08\x96V[a\t\xBC` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\t\x11V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\t\xD1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\n\0W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\n$Wa\n$a\x08\x96V[\x81`@R\x82\x93P\x845\x91Pa\n8\x82a\x08MV[\x90\x82R` \x84\x015\x90a\nJ\x82a\x08MV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\nt\x82a\x08MV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\n\x8EW`\0\x80\xFD[Pa\n\x9B\x85\x82\x86\x01a\t`V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\n\xBAW`\0\x80\xFD[a\n\xC2a\x08\xC5V[\x90P\x815a\n\xCF\x81a\x08MV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\x07Wa\x0B\x07a\x08\x96V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0B\"W`\0\x80\xFD[\x815` a\x0B7a\x0B2\x83a\n\xEDV[a\t\x11V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x0BVW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x0B\xAFW\x81\x81\x8A\x03\x12\x15a\x0BrW`\0\x80\x81\xFD[a\x0Bza\x08\xC5V[\x815a\x0B\x85\x81a\x08MV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x0B\x9E\x81a\x08MV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x0BZV[P\x90\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xCEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xE6W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x0B\xFAW`\0\x80\xFD[a\x0C\x02a\x08\xEEV[\x825\x82\x81\x11\x15a\x0C\x11W`\0\x80\xFD[a\x0C\x1D\x87\x82\x86\x01a\t\xEEV[\x82RPa\x0C-\x86` \x85\x01a\n\xA8V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x0CDW`\0\x80\xFD[a\x0CP\x87\x82\x86\x01a\x0B\x11V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x0ChW`\0\x80\xFD[a\x0Ct\x87\x82\x86\x01a\t`V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0C\xB4W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0C\x98V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\rQW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\r\x06V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\r\xD8a\x01\xC0\x84\x01\x82a\x0C\x8EV[\x90P` \x84\x01Qa\x0E\x16`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x0EQ\x83\x83a\x0C\xF2V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x0Eo\x82\x82a\x0C\x8EV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15a\x0E\x99W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xB0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0E\xC1W`\0\x80\xFD[\x80Qa\x0E\xCFa\x0B2\x82a\n\xEDV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x88\x84\x11\x15a\x0E\xEEW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x0FJW\x80\x85\x8A\x03\x12\x15a\x0F\x0BW`\0\x80\x81\xFD[a\x0F\x13a\x08\xC5V[\x85Qa\x0F\x1E\x81a\x08MV[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa\x0F7\x81a\x08MV[\x90\x82\x01R\x83R\x93\x84\x01\x93\x91\x85\x01\x91a\x0E\xF3V[P\x97\x96PPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0F\x90W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA5\x02\xDA\xE0x\x0C\xB2\xAA\xA3%\x9D\xB6\xC5\xDF?\xE9\x03\xCD3\x9E#\xDAW#{\x17\xE6\xD4\xF2\xDC\x11\xF2dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKPROTOCOLFEES_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockProtocolFees<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockProtocolFees<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockProtocolFees<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockProtocolFees<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockProtocolFees<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockProtocolFees))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockProtocolFees<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKPROTOCOLFEES_ABI.clone(),
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
                MOCKPROTOCOLFEES_ABI.clone(),
                MOCKPROTOCOLFEES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `setProtocolFeeController` (0x2d771389) function
        pub fn set_protocol_fee_controller(
            &self,
            new_fee_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 119, 19, 137], new_fee_controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `takeFees` (0x6f12f442) function
        pub fn take_fees(
            &self,
            order: ResolvedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash([111, 18, 244, 66], (order,))
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockProtocolFeesEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockProtocolFees<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockProtocolFeesErrors {
        DuplicateFeeOutput(DuplicateFeeOutput),
        FeeTooLarge(FeeTooLarge),
        InvalidFeeToken(InvalidFeeToken),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MockProtocolFeesErrors {
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
                <DuplicateFeeOutput as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DuplicateFeeOutput(decoded));
            }
            if let Ok(decoded) = <FeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeTooLarge(decoded));
            }
            if let Ok(decoded) = <InvalidFeeToken as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidFeeToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockProtocolFeesErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DuplicateFeeOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooLarge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidFeeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MockProtocolFeesErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DuplicateFeeOutput as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FeeTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidFeeToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MockProtocolFeesErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DuplicateFeeOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MockProtocolFeesErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DuplicateFeeOutput> for MockProtocolFeesErrors {
        fn from(value: DuplicateFeeOutput) -> Self {
            Self::DuplicateFeeOutput(value)
        }
    }
    impl ::core::convert::From<FeeTooLarge> for MockProtocolFeesErrors {
        fn from(value: FeeTooLarge) -> Self {
            Self::FeeTooLarge(value)
        }
    }
    impl ::core::convert::From<InvalidFeeToken> for MockProtocolFeesErrors {
        fn from(value: InvalidFeeToken) -> Self {
            Self::InvalidFeeToken(value)
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
    pub enum MockProtocolFeesEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProtocolFeeControllerSetFilter(ProtocolFeeControllerSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockProtocolFeesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MockProtocolFeesEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProtocolFeeControllerSetFilter::decode_log(log) {
                return Ok(MockProtocolFeesEvents::ProtocolFeeControllerSetFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockProtocolFeesEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeControllerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MockProtocolFeesEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeControllerSetFilter> for MockProtocolFeesEvents {
        fn from(value: ProtocolFeeControllerSetFilter) -> Self {
            Self::ProtocolFeeControllerSetFilter(value)
        }
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
    ///Container type for all input parameters for the `takeFees` function with signature `takeFees(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))` and selector `0x6f12f442`
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
        name = "takeFees",
        abi = "takeFees(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))"
    )]
    pub struct TakeFeesCall {
        pub order: ResolvedOrder,
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
    pub enum MockProtocolFeesCalls {
        FeeController(FeeControllerCall),
        Owner(OwnerCall),
        SetProtocolFeeController(SetProtocolFeeControllerCall),
        TakeFees(TakeFeesCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockProtocolFeesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeController(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProtocolFeeController(decoded));
            }
            if let Ok(decoded) = <TakeFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TakeFees(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockProtocolFeesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FeeController(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProtocolFeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TakeFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockProtocolFeesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::TakeFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeControllerCall> for MockProtocolFeesCalls {
        fn from(value: FeeControllerCall) -> Self {
            Self::FeeController(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MockProtocolFeesCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeControllerCall> for MockProtocolFeesCalls {
        fn from(value: SetProtocolFeeControllerCall) -> Self {
            Self::SetProtocolFeeController(value)
        }
    }
    impl ::core::convert::From<TakeFeesCall> for MockProtocolFeesCalls {
        fn from(value: TakeFeesCall) -> Self {
            Self::TakeFees(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MockProtocolFeesCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
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
    ///Container type for all return fields from the `takeFees` function with signature `takeFees(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))` and selector `0x6f12f442`
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
    pub struct TakeFeesReturn(pub ResolvedOrder);
}
