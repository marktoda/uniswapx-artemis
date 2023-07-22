pub use mock_exclusivity_override_lib::*;
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
pub mod mock_exclusivity_override_lib {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkExclusivity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkExclusivity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("exclusive"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("exclusivityEndTime",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pass"),
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
                    ::std::borrow::ToOwned::to_owned("handleOverride"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("handleOverride"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("exclusive"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("exclusivityEndTime",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("exclusivityOverrideBps",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("NoExclusiveOverride"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned("NoExclusiveOverride",),
                    inputs: ::std::vec![],
                },],
            )]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKEXCLUSIVITYOVERRIDELIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\tR\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c-\xD4\xBEF\x14a\0;W\x80c\x8C\xED\x85\xCF\x14a\0dW[`\0\x80\xFD[a\0Na\0I6`\x04a\x05\x9DV[a\0\x87V[`@Qa\0[\x91\x90a\x07]V[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x08\x87V[a\x01\x0BV[`@Q\x90\x15\x15\x81R` \x01a\0[V[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\x02\x85\x85\x85\x85a\x01 V[P\x92\x93\x92PPPV[`\0a\x01\x17\x83\x83a\x01\xCAV[\x90P[\x92\x91PPV[a\x01*\x83\x83a\x01\xCAV[a\x01\xC4W\x80a\x01eW`@Q\x7F\xB9\xEC\x1E\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x01Q`\0[\x81Q\x81\x10\x15a\x01\xC1W`\0\x82\x82\x81Q\x81\x10a\x01\x8AWa\x01\x8Aa\x08\xB3V[` \x02` \x01\x01Q\x90Pa\x01\xB3\x84a'\x10a\x01\xA5\x91\x90a\x08\xE2V[` \x83\x01Q\x90a'\x10a\x02\x15V[` \x90\x91\x01R`\x01\x01a\x01mV[PP[PPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15\x80a\x01\xEEWP\x81B\x11[\x80a\x01\x17WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x14\x90P\x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x02JW`\0\x80\xFD[P\x91\x02\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xA3Wa\x02\xA3a\x02QV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xA3Wa\x02\xA3a\x02QV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x13Wa\x03\x13a\x02QV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03=W`\0\x80\xFD[PV[\x805a\x03K\x81a\x03\x1BV[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x03aW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03{Wa\x03{a\x02QV[a\x03\xAC` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x02\xCCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x03\xC1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x03\xF0W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04\x14Wa\x04\x14a\x02QV[\x81`@R\x82\x93P\x845\x91Pa\x04(\x82a\x03\x1BV[\x90\x82R` \x84\x015\x90a\x04:\x82a\x03\x1BV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x04d\x82a\x03\x1BV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x04~W`\0\x80\xFD[Pa\x04\x8B\x85\x82\x86\x01a\x03PV[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x04\xAAW`\0\x80\xFD[a\x04\xB2a\x02\x80V[\x90P\x815a\x04\xBF\x81a\x03\x1BV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x04\xEEW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\nWa\x05\na\x02QV[a\x05\x18\x81\x83`\x05\x1B\x01a\x02\xCCV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x057W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x05\x90W\x81\x81\x8A\x03\x12\x15a\x05SW`\0\x80\x81\xFD[a\x05[a\x02\x80V[\x815a\x05f\x81a\x03\x1BV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x05\x7F\x81a\x03\x1BV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x05;V[P\x90\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xB3W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xCBW`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a\x05\xDFW`\0\x80\xFD[a\x05\xE7a\x02\xA9V[\x825\x82\x81\x11\x15a\x05\xF6W`\0\x80\xFD[a\x06\x02\x8A\x82\x86\x01a\x03\xDEV[\x82RPa\x06\x12\x89` \x85\x01a\x04\x98V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x06)W`\0\x80\xFD[a\x065\x8A\x82\x86\x01a\x04\xDDV[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x06MW`\0\x80\xFD[a\x06Y\x8A\x82\x86\x01a\x03PV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93Pa\x06z` \x86\x01a\x03@V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x06\xB5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x06\x99V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x07RW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\x07V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x07\xD9a\x01\xC0\x84\x01\x82a\x06\x8FV[\x90P` \x84\x01Qa\x08\x17`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x08R\x83\x83a\x06\xF3V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x08p\x82\x82a\x06\x8FV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x9AW`\0\x80\xFD[\x825a\x08\xA5\x81a\x03\x1BV[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x01\x1AW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xDA\\cK\x9D<\x03\x16\xA4\xE9\xE6K\x99\x8F\x1A\x99\xE3\x0F\x1A\xDB\xD6\xBD\xEDq#\xD0^\xA8\xE3\x07p&dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKEXCLUSIVITYOVERRIDELIB_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c-\xD4\xBEF\x14a\0;W\x80c\x8C\xED\x85\xCF\x14a\0dW[`\0\x80\xFD[a\0Na\0I6`\x04a\x05\x9DV[a\0\x87V[`@Qa\0[\x91\x90a\x07]V[`@Q\x80\x91\x03\x90\xF3[a\0wa\0r6`\x04a\x08\x87V[a\x01\x0BV[`@Q\x90\x15\x15\x81R` \x01a\0[V[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\x02\x85\x85\x85\x85a\x01 V[P\x92\x93\x92PPPV[`\0a\x01\x17\x83\x83a\x01\xCAV[\x90P[\x92\x91PPV[a\x01*\x83\x83a\x01\xCAV[a\x01\xC4W\x80a\x01eW`@Q\x7F\xB9\xEC\x1E\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x01Q`\0[\x81Q\x81\x10\x15a\x01\xC1W`\0\x82\x82\x81Q\x81\x10a\x01\x8AWa\x01\x8Aa\x08\xB3V[` \x02` \x01\x01Q\x90Pa\x01\xB3\x84a'\x10a\x01\xA5\x91\x90a\x08\xE2V[` \x83\x01Q\x90a'\x10a\x02\x15V[` \x90\x91\x01R`\x01\x01a\x01mV[PP[PPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15\x80a\x01\xEEWP\x81B\x11[\x80a\x01\x17WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x14\x90P\x92\x91PPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x02JW`\0\x80\xFD[P\x91\x02\x04\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xA3Wa\x02\xA3a\x02QV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xA3Wa\x02\xA3a\x02QV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x13Wa\x03\x13a\x02QV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03=W`\0\x80\xFD[PV[\x805a\x03K\x81a\x03\x1BV[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x03aW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03{Wa\x03{a\x02QV[a\x03\xAC` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x02\xCCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x03\xC1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x03\xF0W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04\x14Wa\x04\x14a\x02QV[\x81`@R\x82\x93P\x845\x91Pa\x04(\x82a\x03\x1BV[\x90\x82R` \x84\x015\x90a\x04:\x82a\x03\x1BV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x04d\x82a\x03\x1BV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x04~W`\0\x80\xFD[Pa\x04\x8B\x85\x82\x86\x01a\x03PV[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x04\xAAW`\0\x80\xFD[a\x04\xB2a\x02\x80V[\x90P\x815a\x04\xBF\x81a\x03\x1BV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x04\xEEW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\nWa\x05\na\x02QV[a\x05\x18\x81\x83`\x05\x1B\x01a\x02\xCCV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x057W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x05\x90W\x81\x81\x8A\x03\x12\x15a\x05SW`\0\x80\x81\xFD[a\x05[a\x02\x80V[\x815a\x05f\x81a\x03\x1BV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x05\x7F\x81a\x03\x1BV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x05;V[P\x90\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xB3W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xCBW`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a\x05\xDFW`\0\x80\xFD[a\x05\xE7a\x02\xA9V[\x825\x82\x81\x11\x15a\x05\xF6W`\0\x80\xFD[a\x06\x02\x8A\x82\x86\x01a\x03\xDEV[\x82RPa\x06\x12\x89` \x85\x01a\x04\x98V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x06)W`\0\x80\xFD[a\x065\x8A\x82\x86\x01a\x04\xDDV[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x06MW`\0\x80\xFD[a\x06Y\x8A\x82\x86\x01a\x03PV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93Pa\x06z` \x86\x01a\x03@V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x06\xB5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x06\x99V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x07RW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x07\x07V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x07\xD9a\x01\xC0\x84\x01\x82a\x06\x8FV[\x90P` \x84\x01Qa\x08\x17`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x08R\x83\x83a\x06\xF3V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x08p\x82\x82a\x06\x8FV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x9AW`\0\x80\xFD[\x825a\x08\xA5\x81a\x03\x1BV[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x01\x1AW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xDA\\cK\x9D<\x03\x16\xA4\xE9\xE6K\x99\x8F\x1A\x99\xE3\x0F\x1A\xDB\xD6\xBD\xEDq#\xD0^\xA8\xE3\x07p&dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKEXCLUSIVITYOVERRIDELIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockExclusivityOverrideLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockExclusivityOverrideLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockExclusivityOverrideLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockExclusivityOverrideLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockExclusivityOverrideLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockExclusivityOverrideLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockExclusivityOverrideLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKEXCLUSIVITYOVERRIDELIB_ABI.clone(),
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
                MOCKEXCLUSIVITYOVERRIDELIB_ABI.clone(),
                MOCKEXCLUSIVITYOVERRIDELIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkExclusivity` (0x8ced85cf) function
        pub fn check_exclusivity(
            &self,
            exclusive: ::ethers::core::types::Address,
            exclusivity_end_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([140, 237, 133, 207], (exclusive, exclusivity_end_time))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handleOverride` (0x2dd4be46) function
        pub fn handle_override(
            &self,
            order: ResolvedOrder,
            exclusive: ::ethers::core::types::Address,
            exclusivity_end_time: ::ethers::core::types::U256,
            exclusivity_override_bps: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash(
                    [45, 212, 190, 70],
                    (
                        order,
                        exclusive,
                        exclusivity_end_time,
                        exclusivity_override_bps,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MockExclusivityOverrideLib<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NoExclusiveOverride` with signature `NoExclusiveOverride()` and selector `0xb9ec1e96`
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
    #[etherror(name = "NoExclusiveOverride", abi = "NoExclusiveOverride()")]
    pub struct NoExclusiveOverride;
    ///Container type for all input parameters for the `checkExclusivity` function with signature `checkExclusivity(address,uint256)` and selector `0x8ced85cf`
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
    #[ethcall(name = "checkExclusivity", abi = "checkExclusivity(address,uint256)")]
    pub struct CheckExclusivityCall {
        pub exclusive: ::ethers::core::types::Address,
        pub exclusivity_end_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `handleOverride` function with signature `handleOverride(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)` and selector `0x2dd4be46`
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
        name = "handleOverride",
        abi = "handleOverride(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)"
    )]
    pub struct HandleOverrideCall {
        pub order: ResolvedOrder,
        pub exclusive: ::ethers::core::types::Address,
        pub exclusivity_end_time: ::ethers::core::types::U256,
        pub exclusivity_override_bps: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockExclusivityOverrideLibCalls {
        CheckExclusivity(CheckExclusivityCall),
        HandleOverride(HandleOverrideCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockExclusivityOverrideLibCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckExclusivityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckExclusivity(decoded));
            }
            if let Ok(decoded) =
                <HandleOverrideCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HandleOverride(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockExclusivityOverrideLibCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckExclusivity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HandleOverride(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockExclusivityOverrideLibCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckExclusivity(element) => ::core::fmt::Display::fmt(element, f),
                Self::HandleOverride(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckExclusivityCall> for MockExclusivityOverrideLibCalls {
        fn from(value: CheckExclusivityCall) -> Self {
            Self::CheckExclusivity(value)
        }
    }
    impl ::core::convert::From<HandleOverrideCall> for MockExclusivityOverrideLibCalls {
        fn from(value: HandleOverrideCall) -> Self {
            Self::HandleOverride(value)
        }
    }
    ///Container type for all return fields from the `checkExclusivity` function with signature `checkExclusivity(address,uint256)` and selector `0x8ced85cf`
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
    pub struct CheckExclusivityReturn {
        pub pass: bool,
    }
    ///Container type for all return fields from the `handleOverride` function with signature `handleOverride(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)` and selector `0x2dd4be46`
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
    pub struct HandleOverrideReturn(pub ResolvedOrder);
}
