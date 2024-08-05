pub use mock_exclusivity_lib::*;
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
pub mod mock_exclusivity_lib {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("handleExclusiveOverride"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "handleExclusiveOverride",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "exclusivityEndTime",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "exclusivityOverrideBps",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("hasFillingRights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasFillingRights"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exclusive"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "exclusivityEndTime",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("pass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NoExclusiveOverride"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoExclusiveOverride",
                            ),
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
    pub static MOCKEXCLUSIVITYLIB_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\tW\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cY\xF8\xA0T\x14a\0;W\x80c\xD3\xA1A\x0F\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02\x8AV[a\0\x83V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0va\0q6`\x04a\x05\xCDV[a\0\x98V[`@Qa\0Z\x91\x90a\x07\x8EV[`\0a\0\x8F\x83\x83a\x01\x1CV[\x90P[\x92\x91PPV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\x13\x85\x85\x85\x85a\x01gV[P\x92\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15\x80a\x01@WP\x81B\x11[\x80a\0\x8FWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x14\x90P\x92\x91PPV[a\x01q\x83\x83a\x01\x1CV[a\x02\x0BW\x80a\x01\xACW`@Q\x7F\xB9\xEC\x1E\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x01Q`\0[\x81Q\x81\x10\x15a\x02\x08W`\0\x82\x82\x81Q\x81\x10a\x01\xD1Wa\x01\xD1a\x08\xB8V[` \x02` \x01\x01Q\x90Pa\x01\xFA\x84a'\x10a\x01\xEC\x91\x90a\x08\xE7V[` \x83\x01Q\x90a'\x10a\x02\x11V[` \x90\x91\x01R`\x01\x01a\x01\xB4V[PP[PPPPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x02FW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02wW`\0\x80\xFD[PV[\x805a\x02\x85\x81a\x02UV[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x9DW`\0\x80\xFD[\x825a\x02\xA8\x81a\x02UV[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x08Wa\x03\x08a\x02\xB6V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x08Wa\x03\x08a\x02\xB6V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03xWa\x03xa\x02\xB6V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x03\x91W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xABWa\x03\xABa\x02\xB6V[a\x03\xDC` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x031V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x03\xF1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x04 W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04DWa\x04Da\x02\xB6V[\x81`@R\x82\x93P\x845\x91Pa\x04X\x82a\x02UV[\x90\x82R` \x84\x015\x90a\x04j\x82a\x02UV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x04\x94\x82a\x02UV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x04\xAEW`\0\x80\xFD[Pa\x04\xBB\x85\x82\x86\x01a\x03\x80V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x04\xDAW`\0\x80\xFD[a\x04\xE2a\x02\xE5V[\x90P\x815a\x04\xEF\x81a\x02UV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x05\x1EW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05:Wa\x05:a\x02\xB6V[a\x05H\x81\x83`\x05\x1B\x01a\x031V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05gW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x05\xC0W\x81\x81\x8A\x03\x12\x15a\x05\x83W`\0\x80\x81\xFD[a\x05\x8Ba\x02\xE5V[\x815a\x05\x96\x81a\x02UV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x05\xAF\x81a\x02UV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x05kV[P\x90\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xE3W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xFBW`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a\x06\x0FW`\0\x80\xFD[a\x06\x17a\x03\x0EV[\x825\x82\x81\x11\x15a\x06&W`\0\x80\xFD[a\x062\x8A\x82\x86\x01a\x04\x0EV[\x82RPa\x06B\x89` \x85\x01a\x04\xC8V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x06YW`\0\x80\xFD[a\x06e\x8A\x82\x86\x01a\x05\rV[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x06}W`\0\x80\xFD[a\x06\x89\x8A\x82\x86\x01a\x03\x80V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93Pa\x06\xAA` \x86\x01a\x02zV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x06\xE5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x06\xC9V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x07\x83W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x078V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x08\na\x01\xC0\x84\x01\x82a\x06\xBFV[\x90P` \x84\x01Qa\x08H`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x08\x83\x83\x83a\x07#V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x08\xA1\x82\x82a\x06\xBFV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\0\x92W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 I\x02Q\xF0\x1E\x9F\x13\x9DH\xA18\xEB\x0B\xBF.\x10\x9A\x85:\xB2\x0B~S\x03c\xC0\x8AHY\"^\x0EdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKEXCLUSIVITYLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cY\xF8\xA0T\x14a\0;W\x80c\xD3\xA1A\x0F\x14a\0cW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02\x8AV[a\0\x83V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0va\0q6`\x04a\x05\xCDV[a\0\x98V[`@Qa\0Z\x91\x90a\x07\x8EV[`\0a\0\x8F\x83\x83a\x01\x1CV[\x90P[\x92\x91PPV[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91Ra\x01\x13\x85\x85\x85\x85a\x01gV[P\x92\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15\x80a\x01@WP\x81B\x11[\x80a\0\x8FWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x14\x90P\x92\x91PPV[a\x01q\x83\x83a\x01\x1CV[a\x02\x0BW\x80a\x01\xACW`@Q\x7F\xB9\xEC\x1E\x96\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x01Q`\0[\x81Q\x81\x10\x15a\x02\x08W`\0\x82\x82\x81Q\x81\x10a\x01\xD1Wa\x01\xD1a\x08\xB8V[` \x02` \x01\x01Q\x90Pa\x01\xFA\x84a'\x10a\x01\xEC\x91\x90a\x08\xE7V[` \x83\x01Q\x90a'\x10a\x02\x11V[` \x90\x91\x01R`\x01\x01a\x01\xB4V[PP[PPPPV[`\0\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x84\x11\x83\x02\x15\x82\x02a\x02FW`\0\x80\xFD[P\x91\x02\x81\x81\x06\x15\x15\x91\x90\x04\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02wW`\0\x80\xFD[PV[\x805a\x02\x85\x81a\x02UV[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x9DW`\0\x80\xFD[\x825a\x02\xA8\x81a\x02UV[\x94` \x93\x90\x93\x015\x93PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x08Wa\x03\x08a\x02\xB6V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\x08Wa\x03\x08a\x02\xB6V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03xWa\x03xa\x02\xB6V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x03\x91W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xABWa\x03\xABa\x02\xB6V[a\x03\xDC` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x031V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x03\xF1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x04 W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04DWa\x04Da\x02\xB6V[\x81`@R\x82\x93P\x845\x91Pa\x04X\x82a\x02UV[\x90\x82R` \x84\x015\x90a\x04j\x82a\x02UV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x04\x94\x82a\x02UV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x04\xAEW`\0\x80\xFD[Pa\x04\xBB\x85\x82\x86\x01a\x03\x80V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x04\xDAW`\0\x80\xFD[a\x04\xE2a\x02\xE5V[\x90P\x815a\x04\xEF\x81a\x02UV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x05\x1EW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05:Wa\x05:a\x02\xB6V[a\x05H\x81\x83`\x05\x1B\x01a\x031V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05gW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x05\xC0W\x81\x81\x8A\x03\x12\x15a\x05\x83W`\0\x80\x81\xFD[a\x05\x8Ba\x02\xE5V[\x815a\x05\x96\x81a\x02UV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x05\xAF\x81a\x02UV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x05kV[P\x90\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xE3W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xFBW`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a\x06\x0FW`\0\x80\xFD[a\x06\x17a\x03\x0EV[\x825\x82\x81\x11\x15a\x06&W`\0\x80\xFD[a\x062\x8A\x82\x86\x01a\x04\x0EV[\x82RPa\x06B\x89` \x85\x01a\x04\xC8V[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x06YW`\0\x80\xFD[a\x06e\x8A\x82\x86\x01a\x05\rV[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x06}W`\0\x80\xFD[a\x06\x89\x8A\x82\x86\x01a\x03\x80V[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x93Pa\x06\xAA` \x86\x01a\x02zV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x06\xE5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x06\xC9V[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x07\x83W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x078V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x08\na\x01\xC0\x84\x01\x82a\x06\xBFV[\x90P` \x84\x01Qa\x08H`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x08\x83\x83\x83a\x07#V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x08\xA1\x82\x82a\x06\xBFV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\0\x92W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 I\x02Q\xF0\x1E\x9F\x13\x9DH\xA18\xEB\x0B\xBF.\x10\x9A\x85:\xB2\x0B~S\x03c\xC0\x8AHY\"^\x0EdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKEXCLUSIVITYLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockExclusivityLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockExclusivityLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockExclusivityLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockExclusivityLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockExclusivityLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockExclusivityLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockExclusivityLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKEXCLUSIVITYLIB_ABI.clone(),
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
                MOCKEXCLUSIVITYLIB_ABI.clone(),
                MOCKEXCLUSIVITYLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `handleExclusiveOverride` (0xd3a1410f) function
        pub fn handle_exclusive_override(
            &self,
            order: ResolvedOrder,
            exclusive: ::ethers::core::types::Address,
            exclusivity_end_time: ::ethers::core::types::U256,
            exclusivity_override_bps: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash(
                    [211, 161, 65, 15],
                    (order, exclusive, exclusivity_end_time, exclusivity_override_bps),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasFillingRights` (0x59f8a054) function
        pub fn has_filling_rights(
            &self,
            exclusive: ::ethers::core::types::Address,
            exclusivity_end_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([89, 248, 160, 84], (exclusive, exclusivity_end_time))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockExclusivityLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NoExclusiveOverride` with signature `NoExclusiveOverride()` and selector `0xb9ec1e96`
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
    #[etherror(name = "NoExclusiveOverride", abi = "NoExclusiveOverride()")]
    pub struct NoExclusiveOverride;
    ///Container type for all input parameters for the `handleExclusiveOverride` function with signature `handleExclusiveOverride(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)` and selector `0xd3a1410f`
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
        name = "handleExclusiveOverride",
        abi = "handleExclusiveOverride(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)"
    )]
    pub struct HandleExclusiveOverrideCall {
        pub order: ResolvedOrder,
        pub exclusive: ::ethers::core::types::Address,
        pub exclusivity_end_time: ::ethers::core::types::U256,
        pub exclusivity_override_bps: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `hasFillingRights` function with signature `hasFillingRights(address,uint256)` and selector `0x59f8a054`
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
    #[ethcall(name = "hasFillingRights", abi = "hasFillingRights(address,uint256)")]
    pub struct HasFillingRightsCall {
        pub exclusive: ::ethers::core::types::Address,
        pub exclusivity_end_time: ::ethers::core::types::U256,
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
    pub enum MockExclusivityLibCalls {
        HandleExclusiveOverride(HandleExclusiveOverrideCall),
        HasFillingRights(HasFillingRightsCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockExclusivityLibCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HandleExclusiveOverrideCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HandleExclusiveOverride(decoded));
            }
            if let Ok(decoded) = <HasFillingRightsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasFillingRights(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockExclusivityLibCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::HandleExclusiveOverride(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasFillingRights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockExclusivityLibCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::HandleExclusiveOverride(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasFillingRights(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HandleExclusiveOverrideCall> for MockExclusivityLibCalls {
        fn from(value: HandleExclusiveOverrideCall) -> Self {
            Self::HandleExclusiveOverride(value)
        }
    }
    impl ::core::convert::From<HasFillingRightsCall> for MockExclusivityLibCalls {
        fn from(value: HasFillingRightsCall) -> Self {
            Self::HasFillingRights(value)
        }
    }
    ///Container type for all return fields from the `handleExclusiveOverride` function with signature `handleExclusiveOverride(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address,uint256,uint256)` and selector `0xd3a1410f`
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
    pub struct HandleExclusiveOverrideReturn(pub ResolvedOrder);
    ///Container type for all return fields from the `hasFillingRights` function with signature `hasFillingRights(address,uint256)` and selector `0x59f8a054`
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
    pub struct HasFillingRightsReturn {
        pub pass: bool,
    }
}
