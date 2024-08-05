pub use mock_fill_contract_double_execution::*;
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
pub mod mock_fill_contract_double_execution {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor1"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor2"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("other"),
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
                                    name: ::std::borrow::ToOwned::to_owned("otherSignedOrder"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKFILLCONTRACTDOUBLEEXECUTION_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r\x9F8\x03\x80a\r\x9F\x839\x81\x01`@\x81\x90Ra\0/\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x95V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0]W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0uW`\0\x80\xFD[a\0~\x83a\0FV[\x91Pa\0\x8C` \x84\x01a\0FV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x0C\xDFa\0\xC0`\09`\0a\x02\xE0\x01R`\0\x81\x81`e\x01Ra\x02\xB8\x01Ra\x0C\xDF`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cP\xDF-@\x14a\0;W\x80cX]\xA6(\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x04\x05V[a\0cV[\0[a\0Na\0^6`\x04a\x07\xBDV[a\x01\x13V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x83\x83`@Q` \x01a\0\xB1\x91\x90a\n?V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDD\x92\x91\x90a\n\xC7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x0BW=`\0\x80>=`\0\xFD[PPPPPPV[`\0[\x82Q\x81\x10\x15a\x02\xA0W`\0[\x83\x82\x81Q\x81\x10a\x014Wa\x014a\n\xF5V[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x02\x97W`\0\x84\x83\x81Q\x81\x10a\x01\\Wa\x01\\a\n\xF5V[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x01yWa\x01ya\n\xF5V[` \x02` \x01\x01Q\x90Pa\x01\xBB\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90V[\x15a\x01\xD3Wa\x01\xCE3\x82` \x01Qa\x03NV[a\x02\x8EV[\x80Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8C\x91\x90a\x0B$V[P[P`\x01\x01a\x01\"V[P`\x01\x01a\x01\x16V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03JW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x82\x80` \x01\x90Q\x81\x01\x90a\x03.\x91\x90a\x0B\x93V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDD\x91\x90a\x0C:V[PPV[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03\xA8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xADV[``\x91P[PP\x90P\x80a\x03\xE8W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0`@\x82\x84\x03\x12\x15a\x03\xFFW`\0\x80\xFD[P\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\x18W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x040W`\0\x80\xFD[a\x04<\x86\x83\x87\x01a\x03\xEDV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x04RW`\0\x80\xFD[Pa\x04_\x85\x82\x86\x01a\x03\xEDV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xBBWa\x04\xBBa\x04iV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xBBWa\x04\xBBa\x04iV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05+Wa\x05+a\x04iV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05MWa\x05Ma\x04iV[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05yW`\0\x80\xFD[PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\x96Wa\x05\x96a\x04iV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x05\xD3W`\0\x80\xFD[\x815a\x05\xE6a\x05\xE1\x82a\x05|V[a\x04\xE4V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x05\xFBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x06*W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06NWa\x06Na\x04iV[\x81`@R\x82\x93P\x845\x91Pa\x06b\x82a\x05WV[\x90\x82R` \x84\x015\x90a\x06t\x82a\x05WV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x06\x9E\x82a\x05WV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x06\xB8W`\0\x80\xFD[Pa\x06\xC5\x85\x82\x86\x01a\x05\xC2V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x06\xE4W`\0\x80\xFD[a\x06\xECa\x04\x98V[\x90P\x815a\x06\xF9\x81a\x05WV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x07(W`\0\x80\xFD[\x815` a\x078a\x05\xE1\x83a\x053V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07WW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x07\xB0W\x81\x81\x8A\x03\x12\x15a\x07sW`\0\x80\x81\xFD[a\x07{a\x04\x98V[\x815a\x07\x86\x81a\x05WV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x07\x9F\x81a\x05WV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x07[V[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xD0W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xE8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07\xFCW`\0\x80\xFD[\x815` a\x08\x0Ca\x05\xE1\x83a\x053V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x08+W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t\x1CW\x805\x86\x81\x11\x15a\x08GW`\0\x80\x81\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\x08}W`\0\x80\x81\xFD[a\x08\x85a\x04\xC1V[\x86\x83\x015\x89\x81\x11\x15a\x08\x97W`\0\x80\x81\xFD[a\x08\xA5\x8F\x89\x83\x87\x01\x01a\x06\x18V[\x82RPa\x08\xB5\x8E`@\x85\x01a\x06\xD2V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\x08\xCCW`\0\x80\x81\xFD[a\x08\xDA\x8F\x89\x83\x87\x01\x01a\x07\x17V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\x08\xF3W`\0\x80\x81\xFD[a\t\x01\x8F\x89\x83\x87\x01\x01a\x05\xC2V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x08/V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\t3W`\0\x80\xFD[Pa\x04_\x85\x82\x86\x01a\x05\xC2V[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\tuW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x95W`\0\x80\xFD[\x806\x03\x82\x13\x15a\t\xA4W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0a\n\0\x82\x83a\t@V[`@\x85Ra\n\x12`@\x86\x01\x82\x84a\t\xABV[\x91PPa\n\"` \x84\x01\x84a\t@V[\x85\x83\x03` \x87\x01Ra\n5\x83\x82\x84a\t\xABV[\x96\x95PPPPPPV[` \x81R`\0a\nR` \x83\x01\x84a\t\xF4V[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\ntW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\\V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n\x95\x81` \x86\x01` \x86\x01a\nYV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\n\xDA`@\x83\x01\x85a\t\xF4V[\x82\x81\x03` \x84\x01Ra\n\xEC\x81\x85a\n}V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0B6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\nRW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x0BWW`\0\x80\xFD[\x81Qa\x0Bea\x05\xE1\x82a\x05|V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0BzW`\0\x80\xFD[a\x0B\x8B\x82` \x83\x01` \x87\x01a\nYV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xA5W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xBDW`\0\x80\xFD[\x90\x83\x01\x90`@\x82\x86\x03\x12\x15a\x0B\xD1W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x0B\xECWa\x0B\xECa\x04iV[`@R\x82Q\x82\x81\x11\x15a\x0B\xFEW`\0\x80\xFD[a\x0C\n\x87\x82\x86\x01a\x0BFV[\x82RP` \x83\x01Q\x82\x81\x11\x15a\x0C\x1FW`\0\x80\xFD[a\x0C+\x87\x82\x86\x01a\x0BFV[` \x83\x01RP\x95\x94PPPPPV[`@\x81R`\0\x82Q`@\x80\x84\x01Ra\x0CU`\x80\x84\x01\x82a\n}V[\x90P` \x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x83\x03\x01``\x85\x01Ra\x0C\x90\x82\x82a\n}V[\x84\x81\x03` \x95\x86\x01R`\0\x81R\x93\x90\x93\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xA5tS\x01\x18\x85\xB9\x8B\xFA\xB9Xl\xDB\x14&\x92\xE7\xD9T\x8AS\x9B\t\xE8\x88\xDA\x05S\x9A\xABI\xA5dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKFILLCONTRACTDOUBLEEXECUTION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cP\xDF-@\x14a\0;W\x80cX]\xA6(\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x04\x05V[a\0cV[\0[a\0Na\0^6`\x04a\x07\xBDV[a\x01\x13V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x83\x83`@Q` \x01a\0\xB1\x91\x90a\n?V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDD\x92\x91\x90a\n\xC7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xF7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x0BW=`\0\x80>=`\0\xFD[PPPPPPV[`\0[\x82Q\x81\x10\x15a\x02\xA0W`\0[\x83\x82\x81Q\x81\x10a\x014Wa\x014a\n\xF5V[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x02\x97W`\0\x84\x83\x81Q\x81\x10a\x01\\Wa\x01\\a\n\xF5V[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x01yWa\x01ya\n\xF5V[` \x02` \x01\x01Q\x90Pa\x01\xBB\x81`\0\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x90V[\x15a\x01\xD3Wa\x01\xCE3\x82` \x01Qa\x03NV[a\x02\x8EV[\x80Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3`\x04\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8C\x91\x90a\x0B$V[P[P`\x01\x01a\x01\"V[P`\x01\x01a\x01\x16V[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\x03JW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\r3X\x84\x82\x80` \x01\x90Q\x81\x01\x90a\x03.\x91\x90a\x0B\x93V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\0\xDD\x91\x90a\x0C:V[PPV[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03\xA8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03\xADV[``\x91P[PP\x90P\x80a\x03\xE8W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0`@\x82\x84\x03\x12\x15a\x03\xFFW`\0\x80\xFD[P\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x04\x18W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x040W`\0\x80\xFD[a\x04<\x86\x83\x87\x01a\x03\xEDV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x04RW`\0\x80\xFD[Pa\x04_\x85\x82\x86\x01a\x03\xEDV[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xBBWa\x04\xBBa\x04iV[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xBBWa\x04\xBBa\x04iV[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05+Wa\x05+a\x04iV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05MWa\x05Ma\x04iV[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05yW`\0\x80\xFD[PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x05\x96Wa\x05\x96a\x04iV[P`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x05\xD3W`\0\x80\xFD[\x815a\x05\xE6a\x05\xE1\x82a\x05|V[a\x04\xE4V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x05\xFBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x06*W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06NWa\x06Na\x04iV[\x81`@R\x82\x93P\x845\x91Pa\x06b\x82a\x05WV[\x90\x82R` \x84\x015\x90a\x06t\x82a\x05WV[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x06\x9E\x82a\x05WV[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x06\xB8W`\0\x80\xFD[Pa\x06\xC5\x85\x82\x86\x01a\x05\xC2V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x06\xE4W`\0\x80\xFD[a\x06\xECa\x04\x98V[\x90P\x815a\x06\xF9\x81a\x05WV[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x07(W`\0\x80\xFD[\x815` a\x078a\x05\xE1\x83a\x053V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07WW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x07\xB0W\x81\x81\x8A\x03\x12\x15a\x07sW`\0\x80\x81\xFD[a\x07{a\x04\x98V[\x815a\x07\x86\x81a\x05WV[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x07\x9F\x81a\x05WV[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x07[V[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x07\xD0W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xE8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x07\xFCW`\0\x80\xFD[\x815` a\x08\x0Ca\x05\xE1\x83a\x053V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x08+W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t\x1CW\x805\x86\x81\x11\x15a\x08GW`\0\x80\x81\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\x08}W`\0\x80\x81\xFD[a\x08\x85a\x04\xC1V[\x86\x83\x015\x89\x81\x11\x15a\x08\x97W`\0\x80\x81\xFD[a\x08\xA5\x8F\x89\x83\x87\x01\x01a\x06\x18V[\x82RPa\x08\xB5\x8E`@\x85\x01a\x06\xD2V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\x08\xCCW`\0\x80\x81\xFD[a\x08\xDA\x8F\x89\x83\x87\x01\x01a\x07\x17V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\x08\xF3W`\0\x80\x81\xFD[a\t\x01\x8F\x89\x83\x87\x01\x01a\x05\xC2V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x08/V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\t3W`\0\x80\xFD[Pa\x04_\x85\x82\x86\x01a\x05\xC2V[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\tuW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x95W`\0\x80\xFD[\x806\x03\x82\x13\x15a\t\xA4W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0a\n\0\x82\x83a\t@V[`@\x85Ra\n\x12`@\x86\x01\x82\x84a\t\xABV[\x91PPa\n\"` \x84\x01\x84a\t@V[\x85\x83\x03` \x87\x01Ra\n5\x83\x82\x84a\t\xABV[\x96\x95PPPPPPV[` \x81R`\0a\nR` \x83\x01\x84a\t\xF4V[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\ntW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\\V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n\x95\x81` \x86\x01` \x86\x01a\nYV[`\x1F\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\n\xDA`@\x83\x01\x85a\t\xF4V[\x82\x81\x03` \x84\x01Ra\n\xEC\x81\x85a\n}V[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0B6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\nRW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x0BWW`\0\x80\xFD[\x81Qa\x0Bea\x05\xE1\x82a\x05|V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0BzW`\0\x80\xFD[a\x0B\x8B\x82` \x83\x01` \x87\x01a\nYV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xA5W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xBDW`\0\x80\xFD[\x90\x83\x01\x90`@\x82\x86\x03\x12\x15a\x0B\xD1W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x0B\xECWa\x0B\xECa\x04iV[`@R\x82Q\x82\x81\x11\x15a\x0B\xFEW`\0\x80\xFD[a\x0C\n\x87\x82\x86\x01a\x0BFV[\x82RP` \x83\x01Q\x82\x81\x11\x15a\x0C\x1FW`\0\x80\xFD[a\x0C+\x87\x82\x86\x01a\x0BFV[` \x83\x01RP\x95\x94PPPPPV[`@\x81R`\0\x82Q`@\x80\x84\x01Ra\x0CU`\x80\x84\x01\x82a\n}V[\x90P` \x84\x01Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x83\x03\x01``\x85\x01Ra\x0C\x90\x82\x82a\n}V[\x84\x81\x03` \x95\x86\x01R`\0\x81R\x93\x90\x93\x01\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xA5tS\x01\x18\x85\xB9\x8B\xFA\xB9Xl\xDB\x14&\x92\xE7\xD9T\x8AS\x9B\t\xE8\x88\xDA\x05S\x9A\xABI\xA5dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFILLCONTRACTDOUBLEEXECUTION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFillContractDoubleExecution<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFillContractDoubleExecution<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFillContractDoubleExecution<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFillContractDoubleExecution<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFillContractDoubleExecution<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFillContractDoubleExecution))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFillContractDoubleExecution<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFILLCONTRACTDOUBLEEXECUTION_ABI.clone(),
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
                MOCKFILLCONTRACTDOUBLEEXECUTION_ABI.clone(),
                MOCKFILLCONTRACTDOUBLEEXECUTION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `execute` (0x50df2d40) function
        pub fn execute(
            &self,
            order: SignedOrder,
            other: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 223, 45, 64], (order, other))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reactorCallback` (0x585da628) function
        pub fn reactor_callback(
            &self,
            resolved_orders: ::std::vec::Vec<ResolvedOrder>,
            other_signed_order: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 93, 166, 40], (resolved_orders, other_signed_order))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFillContractDoubleExecution<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all input parameters for the `execute` function with signature `execute((bytes,bytes),(bytes,bytes))` and selector `0x50df2d40`
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
    #[ethcall(name = "execute", abi = "execute((bytes,bytes),(bytes,bytes))")]
    pub struct ExecuteCall {
        pub order: SignedOrder,
        pub other: SignedOrder,
    }
    ///Container type for all input parameters for the `reactorCallback` function with signature `reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],bytes)` and selector `0x585da628`
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
        name = "reactorCallback",
        abi = "reactorCallback(((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)[],bytes)"
    )]
    pub struct ReactorCallbackCall {
        pub resolved_orders: ::std::vec::Vec<ResolvedOrder>,
        pub other_signed_order: ::ethers::core::types::Bytes,
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
    pub enum MockFillContractDoubleExecutionCalls {
        Execute(ExecuteCall),
        ReactorCallback(ReactorCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFillContractDoubleExecutionCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactorCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFillContractDoubleExecutionCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReactorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockFillContractDoubleExecutionCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for MockFillContractDoubleExecutionCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall>
    for MockFillContractDoubleExecutionCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
}
