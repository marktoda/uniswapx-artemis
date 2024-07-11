pub use mock_resolved_order_lib::*;
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
pub mod mock_resolved_order_lib {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("validate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validate"),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("filler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidReactor"),
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
    pub static MOCKRESOLVEDORDERLIB_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\xB7\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cX\xC9\xED\xBF\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x04\xA0V[a\0EV[\0[a\0O\x82\x82a\0SV[PPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\0\xA4W`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\0OW\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x01 \x90\x84\x90\x86\x90`\x04\x01a\x06RV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x018W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x01LW=`\0\x80>=`\0\xFD[PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xA6Wa\x01\xA6a\x01TV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xA6Wa\x01\xA6a\x01TV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x16Wa\x02\x16a\x01TV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02@W`\0\x80\xFD[PV[\x805a\x02N\x81a\x02\x1EV[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x02dW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02~Wa\x02~a\x01TV[a\x02\xAF` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x01\xCFV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x02\xC4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x02\xF3W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x03\x17Wa\x03\x17a\x01TV[\x81`@R\x82\x93P\x845\x91Pa\x03+\x82a\x02\x1EV[\x90\x82R` \x84\x015\x90a\x03=\x82a\x02\x1EV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x03g\x82a\x02\x1EV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x03\x81W`\0\x80\xFD[Pa\x03\x8E\x85\x82\x86\x01a\x02SV[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x03\xADW`\0\x80\xFD[a\x03\xB5a\x01\x83V[\x90P\x815a\x03\xC2\x81a\x02\x1EV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x03\xF1W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04\rWa\x04\ra\x01TV[a\x04\x1B\x81\x83`\x05\x1B\x01a\x01\xCFV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x04:W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x04\x93W\x81\x81\x8A\x03\x12\x15a\x04VW`\0\x80\x81\xFD[a\x04^a\x01\x83V[\x815a\x04i\x81a\x02\x1EV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x04\x82\x81a\x02\x1EV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x04>V[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xB3W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xCBW`\0\x80\xFD[\x90\x84\x01\x90`\xE0\x82\x87\x03\x12\x15a\x04\xDFW`\0\x80\xFD[a\x04\xE7a\x01\xACV[\x825\x82\x81\x11\x15a\x04\xF6W`\0\x80\xFD[a\x05\x02\x88\x82\x86\x01a\x02\xE1V[\x82RPa\x05\x12\x87` \x85\x01a\x03\x9BV[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x05)W`\0\x80\xFD[a\x055\x88\x82\x86\x01a\x03\xE0V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x05MW`\0\x80\xFD[a\x05Y\x88\x82\x86\x01a\x02SV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x91Pa\x05z` \x84\x01a\x02CV[\x90P\x92P\x92\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x05\xA9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05\x8DV[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x06GW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\xFCV[P\x94\x95\x94PPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x83R`@` \x84\x01R\x83Q`\xE0`@\x85\x01R\x81\x81Q\x16a\x01 \x85\x01R\x81` \x82\x01Q\x16a\x01@\x85\x01R`@\x81\x01Qa\x01`\x85\x01R``\x81\x01Qa\x01\x80\x85\x01R\x81`\x80\x82\x01Q\x16a\x01\xA0\x85\x01R`\xA0\x81\x01Q\x91PP`\xC0a\x01\xC0\x84\x01Ra\x06\xD6a\x01\xE0\x84\x01\x82a\x05\x83V[` \x85\x81\x01Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01R\x90\x81\x01Q`\x80\x86\x01R`@\x81\x01Q`\xA0\x86\x01R\x90\x91PP`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x80\x85\x84\x03\x01`\xC0\x86\x01Ra\x07J\x83\x83a\x05\xE7V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xE0\x86\x01RPa\x07h\x82\x82a\x05\x83V[\x91PP`\x80\x84\x01Qa\x01\0\x84\x01R\x80\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 6\xBB\x01\xE8D`4\x9E\xB6\xC2Y\xA4;f\xB6)'\xEEZZ\\\x85\x017\xDEM\xD9\xDDf\xCABRdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKRESOLVEDORDERLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cX\xC9\xED\xBF\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x04\xA0V[a\0EV[\0[a\0O\x82\x82a\0SV[PPV[\x81QQs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160\x14a\0\xA4W`@Q\x7FM\xDFJd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81Q`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\0OW\x81Q`\x80\x01Q`@Q\x7Fn\x84\xBA+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90cn\x84\xBA+\x90a\x01 \x90\x84\x90\x86\x90`\x04\x01a\x06RV[`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x018W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x01LW=`\0\x80>=`\0\xFD[PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xA6Wa\x01\xA6a\x01TV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xA6Wa\x01\xA6a\x01TV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x16Wa\x02\x16a\x01TV[`@R\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02@W`\0\x80\xFD[PV[\x805a\x02N\x81a\x02\x1EV[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x02dW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02~Wa\x02~a\x01TV[a\x02\xAF` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x01\xCFV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x02\xC4W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x02\xF3W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x03\x17Wa\x03\x17a\x01TV[\x81`@R\x82\x93P\x845\x91Pa\x03+\x82a\x02\x1EV[\x90\x82R` \x84\x015\x90a\x03=\x82a\x02\x1EV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x03g\x82a\x02\x1EV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x03\x81W`\0\x80\xFD[Pa\x03\x8E\x85\x82\x86\x01a\x02SV[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x03\xADW`\0\x80\xFD[a\x03\xB5a\x01\x83V[\x90P\x815a\x03\xC2\x81a\x02\x1EV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x03\xF1W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x04\rWa\x04\ra\x01TV[a\x04\x1B\x81\x83`\x05\x1B\x01a\x01\xCFV[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x04:W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x04\x93W\x81\x81\x8A\x03\x12\x15a\x04VW`\0\x80\x81\xFD[a\x04^a\x01\x83V[\x815a\x04i\x81a\x02\x1EV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x04\x82\x81a\x02\x1EV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x04>V[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\xB3W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\xCBW`\0\x80\xFD[\x90\x84\x01\x90`\xE0\x82\x87\x03\x12\x15a\x04\xDFW`\0\x80\xFD[a\x04\xE7a\x01\xACV[\x825\x82\x81\x11\x15a\x04\xF6W`\0\x80\xFD[a\x05\x02\x88\x82\x86\x01a\x02\xE1V[\x82RPa\x05\x12\x87` \x85\x01a\x03\x9BV[` \x82\x01R`\x80\x83\x015\x82\x81\x11\x15a\x05)W`\0\x80\xFD[a\x055\x88\x82\x86\x01a\x03\xE0V[`@\x83\x01RP`\xA0\x83\x015\x82\x81\x11\x15a\x05MW`\0\x80\xFD[a\x05Y\x88\x82\x86\x01a\x02SV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x91Pa\x05z` \x84\x01a\x02CV[\x90P\x92P\x92\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x05\xA9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x05\x8DV[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x06GW\x81Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x89R\x84\x82\x01Q\x85\x8A\x01R`@\x91\x82\x01Q\x16\x90\x88\x01R``\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05\xFCV[P\x94\x95\x94PPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x83R`@` \x84\x01R\x83Q`\xE0`@\x85\x01R\x81\x81Q\x16a\x01 \x85\x01R\x81` \x82\x01Q\x16a\x01@\x85\x01R`@\x81\x01Qa\x01`\x85\x01R``\x81\x01Qa\x01\x80\x85\x01R\x81`\x80\x82\x01Q\x16a\x01\xA0\x85\x01R`\xA0\x81\x01Q\x91PP`\xC0a\x01\xC0\x84\x01Ra\x06\xD6a\x01\xE0\x84\x01\x82a\x05\x83V[` \x85\x81\x01Q\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x87\x01R\x90\x81\x01Q`\x80\x86\x01R`@\x81\x01Q`\xA0\x86\x01R\x90\x91PP`@\x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x80\x85\x84\x03\x01`\xC0\x86\x01Ra\x07J\x83\x83a\x05\xE7V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xE0\x86\x01RPa\x07h\x82\x82a\x05\x83V[\x91PP`\x80\x84\x01Qa\x01\0\x84\x01R\x80\x91PP\x93\x92PPPV\xFE\xA2dipfsX\"\x12 6\xBB\x01\xE8D`4\x9E\xB6\xC2Y\xA4;f\xB6)'\xEEZZ\\\x85\x017\xDEM\xD9\xDDf\xCABRdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKRESOLVEDORDERLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockResolvedOrderLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockResolvedOrderLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockResolvedOrderLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockResolvedOrderLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockResolvedOrderLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockResolvedOrderLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockResolvedOrderLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKRESOLVEDORDERLIB_ABI.clone(),
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
                MOCKRESOLVEDORDERLIB_ABI.clone(),
                MOCKRESOLVEDORDERLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `validate` (0x58c9edbf) function
        pub fn validate(
            &self,
            resolved_order: ResolvedOrder,
            filler: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 201, 237, 191], (resolved_order, filler))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockResolvedOrderLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Container type for all input parameters for the `validate` function with signature `validate(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address)` and selector `0x58c9edbf`
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
        name = "validate",
        abi = "validate(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32),address)"
    )]
    pub struct ValidateCall {
        pub resolved_order: ResolvedOrder,
        pub filler: ::ethers::core::types::Address,
    }
}
