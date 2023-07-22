pub use mock_fee_controller::*;
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
pub mod mock_fee_controller {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_feeRecipient"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("feeRecipient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feeRecipient"),
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
                    ::std::borrow::ToOwned::to_owned("fees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("fees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFeeOutputs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFeeOutputs"),
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
                            name: ::std::borrow::ToOwned::to_owned("result"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct OutputToken[]"),
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
                    ::std::borrow::ToOwned::to_owned("setFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
            events: ::core::convert::From::from([(
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
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKFEECONTROLLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0C\xDA8\x03\x80a\x0C\xDA\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x81V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0\xB1V[`\0` \x82\x84\x03\x12\x15a\0\x93W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xAAW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0C\x08a\0\xD2`\09`\0\x81\x81`|\x01Ra\x03}\x01Ra\x0C\x08`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0rW`\x005`\xE0\x1C\x80c\x9B\x9A\xC2\xCB\x11a\0PW\x80c\x9B\x9A\xC2\xCB\x14a\x01\x08W\x80c\xDC~\x98\xDF\x14a\x01AW\x80c\xF2\xFD\xE3\x8B\x14a\x01VW`\0\x80\xFD[\x80cF\x90H@\x14a\0wW\x80c\x8A\xA6\xCF\x03\x14a\0\xC8W\x80c\x8D\xA5\xCB[\x14a\0\xE8W[`\0\x80\xFD[a\0\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\0\xD66`\x04a\x08\xEEV[a\x01iV[`@Qa\0\xBF\x91\x90a\t\xC0V[`\0Ta\0\x9E\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x013a\x01\x166`\x04a\n3V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xBFV[a\x01Ta\x01O6`\x04a\nlV[a\x04\x02V[\0[a\x01Ta\x01d6`\x04a\n\xADV[a\x04\xC1V[``\x81`@\x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x89Wa\x01\x89a\x05\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xF2W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x01\xA7W\x90P[P` \x83\x01QQ\x90\x91P`\0\x80[\x84`@\x01QQ\x81\x10\x15a\x03\xF9W`\0\x85`@\x01Q\x82\x81Q\x81\x10a\x02%Wa\x02%a\n\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16`\0\x90\x81R`\x01\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x90\x91 T\x90\x91P\x80\x15a\x03\xE4W`\0a'\x10\x82\x89`@\x01Q\x86\x81Q\x81\x10a\x02\x8AWa\x02\x8Aa\n\xD1V[` \x02` \x01\x01Q` \x01Qa\x02\xA0\x91\x90a\x0B/V[a\x02\xAA\x91\x90a\x0BLV[\x90P`\0\x80[\x86\x81\x10\x15a\x03:W`\0\x89\x82\x81Q\x81\x10a\x02\xCCWa\x02\xCCa\n\xD1V[` \x02` \x01\x01Q\x90P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x03'W`\x01\x92P\x83\x81` \x01\x81\x81Qa\x03#\x91\x90a\x0B\x87V[\x90RP[P\x80a\x032\x81a\x0B\x9AV[\x91PPa\x02\xB0V[P\x80\x15\x80\x15a\x03IWP`\0\x82\x11[\x15a\x03\xE1W`@Q\x80``\x01`@R\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x88\x87\x81Q\x81\x10a\x03\xC7Wa\x03\xC7a\n\xD1V[` \x02` \x01\x01\x81\x90RP\x85\x80a\x03\xDD\x90a\x0B\x9AV[\x96PP[PP[PP\x80\x80a\x03\xF1\x90a\x0B\x9AV[\x91PPa\x02\0V[P\x82RP\x91\x90PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x90\x95\x16\x82R\x92\x90\x92R\x91\x90 UV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x7FV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x04Wa\x06\x04a\x05\xB2V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x04Wa\x06\x04a\x05\xB2V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06tWa\x06ta\x05\xB2V[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\x9EW`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x06\xB2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xCCWa\x06\xCCa\x05\xB2V[a\x06\xFD` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x06-V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07\x12W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x07AW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x07eWa\x07ea\x05\xB2V[\x81`@R\x82\x93P\x845\x91Pa\x07y\x82a\x06|V[\x90\x82R` \x84\x015\x90a\x07\x8B\x82a\x06|V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x07\xB5\x82a\x06|V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x07\xCFW`\0\x80\xFD[Pa\x07\xDC\x85\x82\x86\x01a\x06\xA1V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x07\xFBW`\0\x80\xFD[a\x08\x03a\x05\xE1V[\x90P\x815a\x08\x10\x81a\x06|V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x08?W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08[Wa\x08[a\x05\xB2V[a\x08i\x81\x83`\x05\x1B\x01a\x06-V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08\x88W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\xE1W\x81\x81\x8A\x03\x12\x15a\x08\xA4W`\0\x80\x81\xFD[a\x08\xACa\x05\xE1V[\x815a\x08\xB7\x81a\x06|V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x08\xD0\x81a\x06|V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08\x8CV[P\x90\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x18W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\t,W`\0\x80\xFD[a\t4a\x06\nV[\x825\x82\x81\x11\x15a\tCW`\0\x80\xFD[a\tO\x87\x82\x86\x01a\x07/V[\x82RPa\t_\x86` \x85\x01a\x07\xE9V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\tvW`\0\x80\xFD[a\t\x82\x87\x82\x86\x01a\x08.V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\t\x9AW`\0\x80\xFD[a\t\xA6\x87\x82\x86\x01a\x06\xA1V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\n&W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x86R\x87\x82\x01Q\x88\x87\x01R\x90\x86\x01Q\x16\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\t\xDDV[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\nFW`\0\x80\xFD[\x825a\nQ\x81a\x06|V[\x91P` \x83\x015a\na\x81a\x06|V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\x81W`\0\x80\xFD[\x835a\n\x8C\x81a\x06|V[\x92P` \x84\x015a\n\x9C\x81a\x06|V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\xBFW`\0\x80\xFD[\x815a\n\xCA\x81a\x06|V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0BFWa\x0BFa\x0B\0V[\x92\x91PPV[`\0\x82a\x0B\x82W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0BFWa\x0BFa\x0B\0V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0B\xCBWa\x0B\xCBa\x0B\0V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xFB\xD2\xB3\xFF\xED\xA5x4\x1BU\x07e\x89.,\xCD\x13\xAFK.\xDD\x9D5\xBEz\x1D\xEE\\\xA3f,HdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKFEECONTROLLER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0rW`\x005`\xE0\x1C\x80c\x9B\x9A\xC2\xCB\x11a\0PW\x80c\x9B\x9A\xC2\xCB\x14a\x01\x08W\x80c\xDC~\x98\xDF\x14a\x01AW\x80c\xF2\xFD\xE3\x8B\x14a\x01VW`\0\x80\xFD[\x80cF\x90H@\x14a\0wW\x80c\x8A\xA6\xCF\x03\x14a\0\xC8W\x80c\x8D\xA5\xCB[\x14a\0\xE8W[`\0\x80\xFD[a\0\x9E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\0\xD66`\x04a\x08\xEEV[a\x01iV[`@Qa\0\xBF\x91\x90a\t\xC0V[`\0Ta\0\x9E\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x013a\x01\x166`\x04a\n3V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xBFV[a\x01Ta\x01O6`\x04a\nlV[a\x04\x02V[\0[a\x01Ta\x01d6`\x04a\n\xADV[a\x04\xC1V[``\x81`@\x01QQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x89Wa\x01\x89a\x05\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xF2W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x01\x91\x01\x81a\x01\xA7W\x90P[P` \x83\x01QQ\x90\x91P`\0\x80[\x84`@\x01QQ\x81\x10\x15a\x03\xF9W`\0\x85`@\x01Q\x82\x81Q\x81\x10a\x02%Wa\x02%a\n\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16`\0\x90\x81R`\x01\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x90\x91 T\x90\x91P\x80\x15a\x03\xE4W`\0a'\x10\x82\x89`@\x01Q\x86\x81Q\x81\x10a\x02\x8AWa\x02\x8Aa\n\xD1V[` \x02` \x01\x01Q` \x01Qa\x02\xA0\x91\x90a\x0B/V[a\x02\xAA\x91\x90a\x0BLV[\x90P`\0\x80[\x86\x81\x10\x15a\x03:W`\0\x89\x82\x81Q\x81\x10a\x02\xCCWa\x02\xCCa\n\xD1V[` \x02` \x01\x01Q\x90P\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x03'W`\x01\x92P\x83\x81` \x01\x81\x81Qa\x03#\x91\x90a\x0B\x87V[\x90RP[P\x80a\x032\x81a\x0B\x9AV[\x91PPa\x02\xB0V[P\x80\x15\x80\x15a\x03IWP`\0\x82\x11[\x15a\x03\xE1W`@Q\x80``\x01`@R\x80\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x88\x87\x81Q\x81\x10a\x03\xC7Wa\x03\xC7a\n\xD1V[` \x02` \x01\x01\x81\x90RP\x85\x80a\x03\xDD\x90a\x0B\x9AV[\x96PP[PP[PP\x80\x80a\x03\xF1\x90a\x0B\x9AV[\x91PPa\x02\0V[P\x82RP\x91\x90PV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04\x88W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x90\x95\x16\x82R\x92\x90\x92R\x91\x90 UV[`\0Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x05BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x7FV[`\0\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x04Wa\x06\x04a\x05\xB2V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x04Wa\x06\x04a\x05\xB2V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06tWa\x06ta\x05\xB2V[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\x9EW`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x06\xB2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xCCWa\x06\xCCa\x05\xB2V[a\x06\xFD` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x06-V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x07\x12W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x07AW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x07eWa\x07ea\x05\xB2V[\x81`@R\x82\x93P\x845\x91Pa\x07y\x82a\x06|V[\x90\x82R` \x84\x015\x90a\x07\x8B\x82a\x06|V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x07\xB5\x82a\x06|V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x07\xCFW`\0\x80\xFD[Pa\x07\xDC\x85\x82\x86\x01a\x06\xA1V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x07\xFBW`\0\x80\xFD[a\x08\x03a\x05\xE1V[\x90P\x815a\x08\x10\x81a\x06|V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x08?W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08[Wa\x08[a\x05\xB2V[a\x08i\x81\x83`\x05\x1B\x01a\x06-V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08\x88W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08\xE1W\x81\x81\x8A\x03\x12\x15a\x08\xA4W`\0\x80\x81\xFD[a\x08\xACa\x05\xE1V[\x815a\x08\xB7\x81a\x06|V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x08\xD0\x81a\x06|V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08\x8CV[P\x90\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x18W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\t,W`\0\x80\xFD[a\t4a\x06\nV[\x825\x82\x81\x11\x15a\tCW`\0\x80\xFD[a\tO\x87\x82\x86\x01a\x07/V[\x82RPa\t_\x86` \x85\x01a\x07\xE9V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\tvW`\0\x80\xFD[a\t\x82\x87\x82\x86\x01a\x08.V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\t\x9AW`\0\x80\xFD[a\t\xA6\x87\x82\x86\x01a\x06\xA1V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\n&W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x86R\x87\x82\x01Q\x88\x87\x01R\x90\x86\x01Q\x16\x85\x85\x01R``\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a\t\xDDV[P\x91\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\nFW`\0\x80\xFD[\x825a\nQ\x81a\x06|V[\x91P` \x83\x015a\na\x81a\x06|V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\x81W`\0\x80\xFD[\x835a\n\x8C\x81a\x06|V[\x92P` \x84\x015a\n\x9C\x81a\x06|V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a\n\xBFW`\0\x80\xFD[\x815a\n\xCA\x81a\x06|V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0BFWa\x0BFa\x0B\0V[\x92\x91PPV[`\0\x82a\x0B\x82W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0BFWa\x0BFa\x0B\0V[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0B\xCBWa\x0B\xCBa\x0B\0V[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xFB\xD2\xB3\xFF\xED\xA5x4\x1BU\x07e\x89.,\xCD\x13\xAFK.\xDD\x9D5\xBEz\x1D\xEE\\\xA3f,HdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFEECONTROLLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockFeeController<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFeeController<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFeeController<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFeeController<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFeeController<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFeeController))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFeeController<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKFEECONTROLLER_ABI.clone(),
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
                MOCKFEECONTROLLER_ABI.clone(),
                MOCKFEECONTROLLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `feeRecipient` (0x46904840) function
        pub fn fee_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([70, 144, 72, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees` (0x9b9ac2cb) function
        pub fn fees(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([155, 154, 194, 203], (token_in, token_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFeeOutputs` (0x8aa6cf03) function
        pub fn get_fee_outputs(
            &self,
            order: ResolvedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<OutputToken>> {
            self.0
                .method_hash([138, 166, 207, 3], (order,))
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
        ///Calls the contract's `setFee` (0xdc7e98df) function
        pub fn set_fee(
            &self,
            token_in: ::ethers::core::types::Address,
            token_out: ::ethers::core::types::Address,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 126, 152, 223], (token_in, token_out, fee))
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
        for MockFeeController<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    ///Container type for all input parameters for the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
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
    #[ethcall(name = "feeRecipient", abi = "feeRecipient()")]
    pub struct FeeRecipientCall;
    ///Container type for all input parameters for the `fees` function with signature `fees(address,address)` and selector `0x9b9ac2cb`
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
    #[ethcall(name = "fees", abi = "fees(address,address)")]
    pub struct FeesCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getFeeOutputs` function with signature `getFeeOutputs(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))` and selector `0x8aa6cf03`
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
        name = "getFeeOutputs",
        abi = "getFeeOutputs(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))"
    )]
    pub struct GetFeeOutputsCall {
        pub order: ResolvedOrder,
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
    ///Container type for all input parameters for the `setFee` function with signature `setFee(address,address,uint256)` and selector `0xdc7e98df`
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
    #[ethcall(name = "setFee", abi = "setFee(address,address,uint256)")]
    pub struct SetFeeCall {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: ::ethers::core::types::U256,
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
    pub enum MockFeeControllerCalls {
        FeeRecipient(FeeRecipientCall),
        Fees(FeesCall),
        GetFeeOutputs(GetFeeOutputsCall),
        Owner(OwnerCall),
        SetFee(SetFeeCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFeeControllerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeRecipient(decoded));
            }
            if let Ok(decoded) = <FeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees(decoded));
            }
            if let Ok(decoded) = <GetFeeOutputsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFeeOutputs(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFee(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFeeControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FeeRecipient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFeeOutputs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockFeeControllerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FeeRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeOutputs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FeeRecipientCall> for MockFeeControllerCalls {
        fn from(value: FeeRecipientCall) -> Self {
            Self::FeeRecipient(value)
        }
    }
    impl ::core::convert::From<FeesCall> for MockFeeControllerCalls {
        fn from(value: FeesCall) -> Self {
            Self::Fees(value)
        }
    }
    impl ::core::convert::From<GetFeeOutputsCall> for MockFeeControllerCalls {
        fn from(value: GetFeeOutputsCall) -> Self {
            Self::GetFeeOutputs(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MockFeeControllerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for MockFeeControllerCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MockFeeControllerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
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
    pub struct FeeRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `fees` function with signature `fees(address,address)` and selector `0x9b9ac2cb`
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
    pub struct FeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getFeeOutputs` function with signature `getFeeOutputs(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))` and selector `0x8aa6cf03`
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
    pub struct GetFeeOutputsReturn {
        pub result: ::std::vec::Vec<OutputToken>,
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
