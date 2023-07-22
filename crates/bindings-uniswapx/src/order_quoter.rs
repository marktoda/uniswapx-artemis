pub use order_quoter::*;
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
pub mod order_quoter {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReactor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reactor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IReactor"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sig"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OrdersLengthIncorrect"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OrdersLengthIncorrect",
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
    pub static ORDERQUOTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0Er\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cA\xD8\x8Di\x14a\0FW\x80cvq\xD0{\x14a\0oW\x80c\x99C\xFA\x89\x14a\0\xB0W[`\0\x80\xFD[a\0Ya\0T6`\x04a\x04\xE2V[a\0\xC5V[`@Qa\0f\x91\x90a\x06\x1EV[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ba\0}6`\x04a\x07HV[`@\x81\x81\x01Q\x90\x91\x01\x01Q\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0fV[a\0\xC3a\0\xBE6`\x04a\t[V[a\x02!V[\0[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`@\x83\x81\x01Q\x84\x01\x81\x01Q\x81Q\x80\x83\x01\x83R\x85\x81R` \x80\x82\x01\x86\x90R\x83Q\x90\x81\x01\x84R`\0\x81R\x92Q\x7F\x05\xAF\xC9w\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92c\x05\xAF\xC9w\x92a\x01\xAF\x92\x910\x91`\x04\x01a\n\xFBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xC9W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xDAWP`\x01[a\x02\x1BW=\x80\x80\x15a\x02\x08W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\rV[``\x91P[Pa\x02\x17\x81a\x02\xC4V[\x91PP[\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14a\x02CW`\0\x80\xFD[\x82Q`\x01\x14a\x02~W`@Q\x7F\x06\xEE\x98x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83`\0\x81Q\x81\x10a\x02\x93Wa\x02\x93a\x0B\x8DV[` \x02` \x01\x01Q`@Q` \x01a\x02\xAB\x91\x90a\x06\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q\x81` \x01\xFD[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x80\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90Ra\x01 \x84\x01\x83\x90R``a\x01@\x85\x01\x81\x90R\x91\x84R\x84Q\x80\x83\x01\x86R\x83\x81R` \x80\x82\x01\x85\x90R\x81\x87\x01\x85\x90R\x85\x01R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82Q\x90\x91\x11\x15a\x03@W\x81Q\x82` \x01\xFD[\x81\x80` \x01\x90Q\x81\x01\x90a\x02\x1B\x91\x90a\rjV[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xABWa\x03\xABa\x03YV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xABWa\x03\xABa\x03YV[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xABWa\x03\xABa\x03YV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04>Wa\x04>a\x03YV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04`Wa\x04`a\x03YV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x04\x9DW`\0\x80\xFD[\x815a\x04\xB0a\x04\xAB\x82a\x04FV[a\x03\xF7V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\xC5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xF5W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\rW`\0\x80\xFD[a\x05\x19\x86\x83\x87\x01a\x04\x8CV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x05/W`\0\x80\xFD[Pa\x05<\x85\x82\x86\x01a\x04\x8CV[\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x05aW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05IV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05\x82\x81` \x86\x01` \x86\x01a\x05FV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x06\x13W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\xC8V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x06\x9Aa\x01\xC0\x84\x01\x82a\x05jV[\x90P` \x84\x01Qa\x06\xD8`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x07\x13\x83\x83a\x05\xB4V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x071\x82\x82a\x05jV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x07ZW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07qW`\0\x80\xFD[a\x07}\x84\x82\x85\x01a\x04\x8CV[\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x9FWa\x07\x9Fa\x03YV[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xCBW`\0\x80\xFD[PV[\x805a\x03T\x81a\x07\xA9V[`\0`\xC0\x82\x84\x03\x12\x15a\x07\xEBW`\0\x80\xFD[a\x07\xF3a\x03\x88V[\x90P\x815a\x08\0\x81a\x07\xA9V[\x81R` \x82\x015a\x08\x10\x81a\x07\xA9V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a\x089\x81a\x07\xA9V[`\x80\x82\x01R`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08XW`\0\x80\xFD[a\x08d\x84\x82\x85\x01a\x04\x8CV[`\xA0\x83\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x08\x82W`\0\x80\xFD[a\x08\x8Aa\x03\xB1V[\x90P\x815a\x08\x97\x81a\x07\xA9V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x08\xC6W`\0\x80\xFD[\x815` a\x08\xD6a\x04\xAB\x83a\x07\x85V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08\xF5W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\tNW\x81\x81\x8A\x03\x12\x15a\t\x11W`\0\x80\x81\xFD[a\t\x19a\x03\xB1V[\x815a\t$\x81a\x07\xA9V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\t=\x81a\x07\xA9V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08\xF9V[P\x90\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\tpW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x88W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\t\x9CW`\0\x80\xFD[\x815` a\t\xACa\x04\xAB\x83a\x07\x85V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\t\xCBW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\n\xBCW\x805\x86\x81\x11\x15a\t\xE6W`\0\x80\xFD[\x87\x01`\xE0\x81\x8E\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x12\x15a\n\x1AW`\0\x80\xFD[a\n\"a\x03\xD4V[\x85\x82\x015\x88\x81\x11\x15a\n3W`\0\x80\xFD[a\nA\x8F\x88\x83\x86\x01\x01a\x07\xD9V[\x82RPa\nQ\x8E`@\x84\x01a\x08pV[\x86\x82\x01R`\xA0\x82\x015\x88\x81\x11\x15a\nhW`\0\x80\x81\xFD[a\nv\x8F\x88\x83\x86\x01\x01a\x08\xB5V[`@\x83\x01RP`\xC0\x82\x015\x88\x81\x11\x15a\n\x8FW`\0\x80\x81\xFD[a\n\x9D\x8F\x88\x83\x86\x01\x01a\x04\x8CV[``\x83\x01RP`\xE0\x91\x90\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\t\xCFV[P\x97Pa\n\xCC\x90P\x88\x82\x01a\x07\xCEV[\x95PPP`@\x86\x015\x91P\x80\x82\x11\x15a\n\xE4W`\0\x80\xFD[Pa\n\xF1\x86\x82\x87\x01a\x04\x8CV[\x91PP\x92P\x92P\x92V[``\x81R`\0\x84Q`@``\x84\x01Ra\x0B\x17`\xA0\x84\x01\x82a\x05jV[\x90P` \x86\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x84\x83\x03\x01`\x80\x85\x01Ra\x0BR\x82\x82a\x05jV[\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x84\x01R\x82\x81\x03`@\x84\x01Ra\x0B\x83\x81\x85a\x05jV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0B\xCDW`\0\x80\xFD[\x81Qa\x0B\xDBa\x04\xAB\x82a\x04FV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B\xF0W`\0\x80\xFD[a\x07}\x82` \x83\x01` \x87\x01a\x05FV[`\0`\xC0\x82\x84\x03\x12\x15a\x0C\x13W`\0\x80\xFD[a\x0C\x1Ba\x03\x88V[\x90P\x81Qa\x0C(\x81a\x07\xA9V[\x81R` \x82\x01Qa\x0C8\x81a\x07\xA9V[\x80` \x83\x01RP`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa\x0Ca\x81a\x07\xA9V[`\x80\x82\x01R`\xA0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x80W`\0\x80\xFD[a\x08d\x84\x82\x85\x01a\x0B\xBCV[`\0``\x82\x84\x03\x12\x15a\x0C\x9EW`\0\x80\xFD[a\x0C\xA6a\x03\xB1V[\x90P\x81Qa\x0C\xB3\x81a\x07\xA9V[\x80\x82RP` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x0C\xE2W`\0\x80\xFD[\x81Q` a\x0C\xF2a\x04\xAB\x83a\x07\x85V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\r\x11W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\tNW\x81\x81\x8A\x03\x12\x15a\r-W`\0\x80\x81\xFD[a\r5a\x03\xB1V[\x81Qa\r@\x81a\x07\xA9V[\x81R\x81\x86\x01Q\x86\x82\x01R`@\x80\x83\x01Qa\rY\x81a\x07\xA9V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\r\x15V[`\0` \x82\x84\x03\x12\x15a\r|W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x94W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\r\xA8W`\0\x80\xFD[a\r\xB0a\x03\xD4V[\x82Q\x82\x81\x11\x15a\r\xBFW`\0\x80\xFD[a\r\xCB\x87\x82\x86\x01a\x0C\x01V[\x82RPa\r\xDB\x86` \x85\x01a\x0C\x8CV[` \x82\x01R`\x80\x83\x01Q\x82\x81\x11\x15a\r\xF2W`\0\x80\xFD[a\r\xFE\x87\x82\x86\x01a\x0C\xD1V[`@\x83\x01RP`\xA0\x83\x01Q\x82\x81\x11\x15a\x0E\x16W`\0\x80\xFD[a\x0E\"\x87\x82\x86\x01a\x0B\xBCV[``\x83\x01RP`\xC0\x92\x90\x92\x01Q`\x80\x83\x01RP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD8\x02\xF2LZ1\xDD\x0B\xF7*N\x8A\xC1:\xE7H\x85n\"\x89\xD8\xA1C\xBE\xB2p\xED\xBD\x1D\x7FO\xAEdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ORDERQUOTER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cA\xD8\x8Di\x14a\0FW\x80cvq\xD0{\x14a\0oW\x80c\x99C\xFA\x89\x14a\0\xB0W[`\0\x80\xFD[a\0Ya\0T6`\x04a\x04\xE2V[a\0\xC5V[`@Qa\0f\x91\x90a\x06\x1EV[`@Q\x80\x91\x03\x90\xF3[a\0\x8Ba\0}6`\x04a\x07HV[`@\x81\x81\x01Q\x90\x91\x01\x01Q\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0fV[a\0\xC3a\0\xBE6`\x04a\t[V[a\x02!V[\0[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x82\x90Ra\x01\0\x83\x01\x82\x90Ra\x01 \x83\x01\x82\x90R``a\x01@\x84\x01\x81\x90R\x90\x83R\x83Q\x80\x82\x01\x85R\x82\x81R` \x80\x82\x01\x84\x90R\x81\x86\x01\x84\x90R\x84\x01R\x92\x82\x01\x83\x90R\x82\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`@\x83\x81\x01Q\x84\x01\x81\x01Q\x81Q\x80\x83\x01\x83R\x85\x81R` \x80\x82\x01\x86\x90R\x83Q\x90\x81\x01\x84R`\0\x81R\x92Q\x7F\x05\xAF\xC9w\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x92c\x05\xAF\xC9w\x92a\x01\xAF\x92\x910\x91`\x04\x01a\n\xFBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xC9W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x01\xDAWP`\x01[a\x02\x1BW=\x80\x80\x15a\x02\x08W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x02\rV[``\x91P[Pa\x02\x17\x81a\x02\xC4V[\x91PP[\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x160\x14a\x02CW`\0\x80\xFD[\x82Q`\x01\x14a\x02~W`@Q\x7F\x06\xEE\x98x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83`\0\x81Q\x81\x10a\x02\x93Wa\x02\x93a\x0B\x8DV[` \x02` \x01\x01Q`@Q` \x01a\x02\xAB\x91\x90a\x06\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x80Q\x81` \x01\xFD[`@\x80Qa\x01`\x81\x01\x82R`\0`\xA0\x82\x01\x81\x81R`\xC0\x80\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90Ra\x01 \x84\x01\x83\x90R``a\x01@\x85\x01\x81\x90R\x91\x84R\x84Q\x80\x83\x01\x86R\x83\x81R` \x80\x82\x01\x85\x90R\x81\x87\x01\x85\x90R\x85\x01R\x93\x83\x01\x81\x90R\x80\x83\x01R`\x80\x82\x01R\x82Q\x90\x91\x11\x15a\x03@W\x81Q\x82` \x01\xFD[\x81\x80` \x01\x90Q\x81\x01\x90a\x02\x1B\x91\x90a\rjV[\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xABWa\x03\xABa\x03YV[`@R\x90V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xABWa\x03\xABa\x03YV[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xABWa\x03\xABa\x03YV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04>Wa\x04>a\x03YV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04`Wa\x04`a\x03YV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x04\x9DW`\0\x80\xFD[\x815a\x04\xB0a\x04\xAB\x82a\x04FV[a\x03\xF7V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\xC5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xF5W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\rW`\0\x80\xFD[a\x05\x19\x86\x83\x87\x01a\x04\x8CV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x05/W`\0\x80\xFD[Pa\x05<\x85\x82\x86\x01a\x04\x8CV[\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x05aW\x81\x81\x01Q\x83\x82\x01R` \x01a\x05IV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x05\x82\x81` \x86\x01` \x86\x01a\x05FV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x06\x13W\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\xC8V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`\xE0` \x84\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82Q\x16a\x01\0\x85\x01R\x80` \x83\x01Q\x16a\x01 \x85\x01R`@\x82\x01Qa\x01@\x85\x01R``\x82\x01Qa\x01`\x85\x01R\x80`\x80\x83\x01Q\x16a\x01\x80\x85\x01RP`\xA0\x81\x01Q\x90P`\xC0a\x01\xA0\x84\x01Ra\x06\x9Aa\x01\xC0\x84\x01\x82a\x05jV[\x90P` \x84\x01Qa\x06\xD8`@\x85\x01\x82\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q\x91\x01RV[P`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x80\x85\x84\x03\x01`\xA0\x86\x01Ra\x07\x13\x83\x83a\x05\xB4V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPa\x071\x82\x82a\x05jV[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x07ZW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07qW`\0\x80\xFD[a\x07}\x84\x82\x85\x01a\x04\x8CV[\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x9FWa\x07\x9Fa\x03YV[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07\xCBW`\0\x80\xFD[PV[\x805a\x03T\x81a\x07\xA9V[`\0`\xC0\x82\x84\x03\x12\x15a\x07\xEBW`\0\x80\xFD[a\x07\xF3a\x03\x88V[\x90P\x815a\x08\0\x81a\x07\xA9V[\x81R` \x82\x015a\x08\x10\x81a\x07\xA9V[\x80` \x83\x01RP`@\x82\x015`@\x82\x01R``\x82\x015``\x82\x01R`\x80\x82\x015a\x089\x81a\x07\xA9V[`\x80\x82\x01R`\xA0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08XW`\0\x80\xFD[a\x08d\x84\x82\x85\x01a\x04\x8CV[`\xA0\x83\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x08\x82W`\0\x80\xFD[a\x08\x8Aa\x03\xB1V[\x90P\x815a\x08\x97\x81a\x07\xA9V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x08\xC6W`\0\x80\xFD[\x815` a\x08\xD6a\x04\xAB\x83a\x07\x85V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08\xF5W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\tNW\x81\x81\x8A\x03\x12\x15a\t\x11W`\0\x80\x81\xFD[a\t\x19a\x03\xB1V[\x815a\t$\x81a\x07\xA9V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\t=\x81a\x07\xA9V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08\xF9V[P\x90\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\tpW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x88W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\t\x9CW`\0\x80\xFD[\x815` a\t\xACa\x04\xAB\x83a\x07\x85V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\t\xCBW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\n\xBCW\x805\x86\x81\x11\x15a\t\xE6W`\0\x80\xFD[\x87\x01`\xE0\x81\x8E\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x12\x15a\n\x1AW`\0\x80\xFD[a\n\"a\x03\xD4V[\x85\x82\x015\x88\x81\x11\x15a\n3W`\0\x80\xFD[a\nA\x8F\x88\x83\x86\x01\x01a\x07\xD9V[\x82RPa\nQ\x8E`@\x84\x01a\x08pV[\x86\x82\x01R`\xA0\x82\x015\x88\x81\x11\x15a\nhW`\0\x80\x81\xFD[a\nv\x8F\x88\x83\x86\x01\x01a\x08\xB5V[`@\x83\x01RP`\xC0\x82\x015\x88\x81\x11\x15a\n\x8FW`\0\x80\x81\xFD[a\n\x9D\x8F\x88\x83\x86\x01\x01a\x04\x8CV[``\x83\x01RP`\xE0\x91\x90\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\t\xCFV[P\x97Pa\n\xCC\x90P\x88\x82\x01a\x07\xCEV[\x95PPP`@\x86\x015\x91P\x80\x82\x11\x15a\n\xE4W`\0\x80\xFD[Pa\n\xF1\x86\x82\x87\x01a\x04\x8CV[\x91PP\x92P\x92P\x92V[``\x81R`\0\x84Q`@``\x84\x01Ra\x0B\x17`\xA0\x84\x01\x82a\x05jV[\x90P` \x86\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x84\x83\x03\x01`\x80\x85\x01Ra\x0BR\x82\x82a\x05jV[\x91PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16` \x84\x01R\x82\x81\x03`@\x84\x01Ra\x0B\x83\x81\x85a\x05jV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0B\xCDW`\0\x80\xFD[\x81Qa\x0B\xDBa\x04\xAB\x82a\x04FV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B\xF0W`\0\x80\xFD[a\x07}\x82` \x83\x01` \x87\x01a\x05FV[`\0`\xC0\x82\x84\x03\x12\x15a\x0C\x13W`\0\x80\xFD[a\x0C\x1Ba\x03\x88V[\x90P\x81Qa\x0C(\x81a\x07\xA9V[\x81R` \x82\x01Qa\x0C8\x81a\x07\xA9V[\x80` \x83\x01RP`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa\x0Ca\x81a\x07\xA9V[`\x80\x82\x01R`\xA0\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x80W`\0\x80\xFD[a\x08d\x84\x82\x85\x01a\x0B\xBCV[`\0``\x82\x84\x03\x12\x15a\x0C\x9EW`\0\x80\xFD[a\x0C\xA6a\x03\xB1V[\x90P\x81Qa\x0C\xB3\x81a\x07\xA9V[\x80\x82RP` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x0C\xE2W`\0\x80\xFD[\x81Q` a\x0C\xF2a\x04\xAB\x83a\x07\x85V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\r\x11W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\tNW\x81\x81\x8A\x03\x12\x15a\r-W`\0\x80\x81\xFD[a\r5a\x03\xB1V[\x81Qa\r@\x81a\x07\xA9V[\x81R\x81\x86\x01Q\x86\x82\x01R`@\x80\x83\x01Qa\rY\x81a\x07\xA9V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\r\x15V[`\0` \x82\x84\x03\x12\x15a\r|W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x94W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\r\xA8W`\0\x80\xFD[a\r\xB0a\x03\xD4V[\x82Q\x82\x81\x11\x15a\r\xBFW`\0\x80\xFD[a\r\xCB\x87\x82\x86\x01a\x0C\x01V[\x82RPa\r\xDB\x86` \x85\x01a\x0C\x8CV[` \x82\x01R`\x80\x83\x01Q\x82\x81\x11\x15a\r\xF2W`\0\x80\xFD[a\r\xFE\x87\x82\x86\x01a\x0C\xD1V[`@\x83\x01RP`\xA0\x83\x01Q\x82\x81\x11\x15a\x0E\x16W`\0\x80\xFD[a\x0E\"\x87\x82\x86\x01a\x0B\xBCV[``\x83\x01RP`\xC0\x92\x90\x92\x01Q`\x80\x83\x01RP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD8\x02\xF2LZ1\xDD\x0B\xF7*N\x8A\xC1:\xE7H\x85n\"\x89\xD8\xA1C\xBE\xB2p\xED\xBD\x1D\x7FO\xAEdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static ORDERQUOTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct OrderQuoter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OrderQuoter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OrderQuoter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OrderQuoter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OrderQuoter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OrderQuoter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OrderQuoter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ORDERQUOTER_ABI.clone(),
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
                ORDERQUOTER_ABI.clone(),
                ORDERQUOTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getReactor` (0x7671d07b) function
        pub fn get_reactor(
            &self,
            order: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([118, 113, 208, 123], order)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quote` (0x41d88d69) function
        pub fn quote(
            &self,
            order: ::ethers::core::types::Bytes,
            sig: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ResolvedOrder> {
            self.0
                .method_hash([65, 216, 141, 105], (order, sig))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactorCallback` (0x9943fa89) function
        pub fn reactor_callback(
            &self,
            resolved_orders: ::std::vec::Vec<ResolvedOrder>,
            filler: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 67, 250, 137], (resolved_orders, filler, p2))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for OrderQuoter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `OrdersLengthIncorrect` with signature `OrdersLengthIncorrect()` and selector `0x06ee9878`
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
    #[etherror(name = "OrdersLengthIncorrect", abi = "OrdersLengthIncorrect()")]
    pub struct OrdersLengthIncorrect;
    ///Container type for all input parameters for the `getReactor` function with signature `getReactor(bytes)` and selector `0x7671d07b`
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
    #[ethcall(name = "getReactor", abi = "getReactor(bytes)")]
    pub struct GetReactorCall {
        pub order: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `quote` function with signature `quote(bytes,bytes)` and selector `0x41d88d69`
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
    #[ethcall(name = "quote", abi = "quote(bytes,bytes)")]
    pub struct QuoteCall {
        pub order: ::ethers::core::types::Bytes,
        pub sig: ::ethers::core::types::Bytes,
    }
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
        pub p2: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OrderQuoterCalls {
        GetReactor(GetReactorCall),
        Quote(QuoteCall),
        ReactorCallback(ReactorCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for OrderQuoterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetReactorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReactor(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) =
                <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReactorCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OrderQuoterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetReactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReactorCallback(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OrderQuoterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetReactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetReactorCall> for OrderQuoterCalls {
        fn from(value: GetReactorCall) -> Self {
            Self::GetReactor(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for OrderQuoterCalls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall> for OrderQuoterCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
    ///Container type for all return fields from the `getReactor` function with signature `getReactor(bytes)` and selector `0x7671d07b`
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
    pub struct GetReactorReturn {
        pub reactor: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `quote` function with signature `quote(bytes,bytes)` and selector `0x41d88d69`
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
    pub struct QuoteReturn {
        pub result: ResolvedOrder,
    }
}
