pub use mock_fill_contract_with_output_override::*;
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
pub mod mock_fill_contract_with_output_override {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_reactor"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("setOutputAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOutputAmount"),
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
    pub static MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r\x108\x03\x80a\r\x10\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0Cqa\0\x9F`\09`\0\x81\x81`\xDC\x01R\x81\x81a\x01\x86\x01R\x81\x81a\x02\x9D\x01Ra\x03\x06\x01Ra\x0Cq`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\rz\x16\xC3\x14a\0QW\x80c$\xF9\xC44\x14a\0fW\x80c?b\x19.\x14a\0yW\x80cX]\xA6(\x14a\0\x8CW[`\0\x80\xFD[a\0da\0_6`\x04a\x04VV[a\0\x9FV[\0[a\0da\0t6`\x04a\x04\xCBV[`\0UV[a\0da\0\x876`\x04a\x04\xE4V[a\x01IV[a\0da\0\x9A6`\x04a\x08qV[a\x01\xF0V[`@Q\x7F\x13\xFBr\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13\xFBr\xC7\x90a\x01\x13\x90\x85\x90\x85\x90`\x04\x01a\n\xFDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01AW=`\0\x80>=`\0\xFD[PPPPPPV[`@Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\r3X\x84\x90a\x01\xBB\x90\x84\x90`\x04\x01a\x0B\xC0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xD5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xE9W=`\0\x80>=`\0\xFD[PPPPPV[`\0[\x82Q\x81\x10\x15a\x03\xB7W`\0[\x83\x82\x81Q\x81\x10a\x02\x11Wa\x02\x11a\x0B\xEAV[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x03\xAEW`\0\x84\x83\x81Q\x81\x10a\x029Wa\x029a\x0B\xEAV[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x02VWa\x02Va\x0B\xEAV[` \x02` \x01\x01Q\x90P`\0\x80T`\0\x14a\x02sW`\0Ta\x02yV[\x81` \x01Q[\x82Q\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xC7Wa\x02\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x03\xBCV[a\x03\xA4V[\x81Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x83\x01R\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA2\x91\x90a\x0C\x19V[P[PP`\x01\x01a\x01\xFFV[P`\x01\x01a\x01\xF3V[PPPV[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\x16W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\x1BV[``\x91P[PP\x90P\x80a\x03\xB7W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x04iW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x81W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\x95W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xA4W`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x04\xB9W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xDDW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\xF6W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\rW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x05\x1FW`\0\x80\xFD[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05xWa\x05xa\x05&V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05xWa\x05xa\x05&V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xE8Wa\x05\xE8a\x05&V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\nWa\x06\na\x05&V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x066W`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x06JW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06dWa\x06da\x05&V[a\x06\x95` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x05\xA1V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\xAAW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x06\xD9W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06\xFDWa\x06\xFDa\x05&V[\x81`@R\x82\x93P\x845\x91Pa\x07\x11\x82a\x06\x14V[\x90\x82R` \x84\x015\x90a\x07#\x82a\x06\x14V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x07M\x82a\x06\x14V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x07gW`\0\x80\xFD[Pa\x07t\x85\x82\x86\x01a\x069V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x07\x93W`\0\x80\xFD[a\x07\x9Ba\x05UV[\x90P\x815a\x07\xA8\x81a\x06\x14V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x07\xD7W`\0\x80\xFD[\x815` a\x07\xECa\x07\xE7\x83a\x05\xF0V[a\x05\xA1V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08\x0BW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08dW\x81\x81\x8A\x03\x12\x15a\x08'W`\0\x80\x81\xFD[a\x08/a\x05UV[\x815a\x08:\x81a\x06\x14V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x08S\x81a\x06\x14V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08\x0FV[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x84W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x9CW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08\xB0W`\0\x80\xFD[\x815` a\x08\xC0a\x07\xE7\x83a\x05\xF0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x08\xDFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t\xD0W\x805\x86\x81\x11\x15a\x08\xFBW`\0\x80\x81\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\t1W`\0\x80\x81\xFD[a\t9a\x05~V[\x86\x83\x015\x89\x81\x11\x15a\tKW`\0\x80\x81\xFD[a\tY\x8F\x89\x83\x87\x01\x01a\x06\xC7V[\x82RPa\ti\x8E`@\x85\x01a\x07\x81V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\t\x80W`\0\x80\x81\xFD[a\t\x8E\x8F\x89\x83\x87\x01\x01a\x07\xC6V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\t\xA7W`\0\x80\x81\xFD[a\t\xB5\x8F\x89\x83\x87\x01\x01a\x069V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x08\xE3V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\t\xE7W`\0\x80\xFD[Pa\t\xF4\x85\x82\x86\x01a\x069V[\x91PP\x92P\x92\x90PV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\n3W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nSW`\0\x80\xFD[\x806\x03\x82\x13\x15a\nbW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0a\n\xBE\x82\x83a\t\xFEV[`@\x85Ra\n\xD0`@\x86\x01\x82\x84a\niV[\x91PPa\n\xE0` \x84\x01\x84a\t\xFEV[\x85\x83\x03` \x87\x01Ra\n\xF3\x83\x82\x84a\niV[\x96\x95PPPPPPV[`@\x80\x82R\x81\x01\x82\x90R`\0```\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83\x80[\x87\x81\x10\x15a\x0B\x9DW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x8A6\x03\x01\x81\x12a\x0B{W\x82\x83\xFD[a\x0B\x87\x86\x8B\x83\x01a\n\xB2V[\x95PP` \x93\x84\x01\x93\x92\x90\x92\x01\x91`\x01\x01a\x0B\x1BV[PPPP\x82\x81\x03` \x84\x01Ra\x0B\xB7\x81`\0\x81R` \x01\x90V[\x95\x94PPPPPV[`@\x81R`\0a\x0B\xD3`@\x83\x01\x84a\n\xB2V[\x82\x81\x03` \x93\x84\x01R`\0\x81R\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C+W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\x1FW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xD64\xC6\x7Ff\xA8\x1E\xAC\x0C\xB0Lu\xA1x\xD5h\x8By\xB1\x8Ej\x81\xCAO@\x1E\xDA8gAj\x85dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\rz\x16\xC3\x14a\0QW\x80c$\xF9\xC44\x14a\0fW\x80c?b\x19.\x14a\0yW\x80cX]\xA6(\x14a\0\x8CW[`\0\x80\xFD[a\0da\0_6`\x04a\x04VV[a\0\x9FV[\0[a\0da\0t6`\x04a\x04\xCBV[`\0UV[a\0da\0\x876`\x04a\x04\xE4V[a\x01IV[a\0da\0\x9A6`\x04a\x08qV[a\x01\xF0V[`@Q\x7F\x13\xFBr\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13\xFBr\xC7\x90a\x01\x13\x90\x85\x90\x85\x90`\x04\x01a\n\xFDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01AW=`\0\x80>=`\0\xFD[PPPPPPV[`@Q\x7F\r3X\x84\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\r3X\x84\x90a\x01\xBB\x90\x84\x90`\x04\x01a\x0B\xC0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xD5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xE9W=`\0\x80>=`\0\xFD[PPPPPV[`\0[\x82Q\x81\x10\x15a\x03\xB7W`\0[\x83\x82\x81Q\x81\x10a\x02\x11Wa\x02\x11a\x0B\xEAV[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x03\xAEW`\0\x84\x83\x81Q\x81\x10a\x029Wa\x029a\x0B\xEAV[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\x02VWa\x02Va\x0B\xEAV[` \x02` \x01\x01Q\x90P`\0\x80T`\0\x14a\x02sW`\0Ta\x02yV[\x81` \x01Q[\x82Q\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x02\xC7Wa\x02\xC2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x03\xBCV[a\x03\xA4V[\x81Q`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$\x83\x01R\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA2\x91\x90a\x0C\x19V[P[PP`\x01\x01a\x01\xFFV[P`\x01\x01a\x01\xF3V[PPPV[`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04\x16W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04\x1BV[``\x91P[PP\x90P\x80a\x03\xB7W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80` \x83\x85\x03\x12\x15a\x04iW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x81W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\x95W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xA4W`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x04\xB9W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xDDW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\xF6W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\rW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x05\x1FW`\0\x80\xFD[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05xWa\x05xa\x05&V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05xWa\x05xa\x05&V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xE8Wa\x05\xE8a\x05&V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\nWa\x06\na\x05&V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x066W`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x06JW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06dWa\x06da\x05&V[a\x06\x95` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x05\xA1V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x06\xAAW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x06\xD9W`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x06\xFDWa\x06\xFDa\x05&V[\x81`@R\x82\x93P\x845\x91Pa\x07\x11\x82a\x06\x14V[\x90\x82R` \x84\x015\x90a\x07#\x82a\x06\x14V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x07M\x82a\x06\x14V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x07gW`\0\x80\xFD[Pa\x07t\x85\x82\x86\x01a\x069V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x07\x93W`\0\x80\xFD[a\x07\x9Ba\x05UV[\x90P\x815a\x07\xA8\x81a\x06\x14V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x07\xD7W`\0\x80\xFD[\x815` a\x07\xECa\x07\xE7\x83a\x05\xF0V[a\x05\xA1V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x08\x0BW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x08dW\x81\x81\x8A\x03\x12\x15a\x08'W`\0\x80\x81\xFD[a\x08/a\x05UV[\x815a\x08:\x81a\x06\x14V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x08S\x81a\x06\x14V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x08\x0FV[P\x90\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x84W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x08\x9CW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x08\xB0W`\0\x80\xFD[\x815` a\x08\xC0a\x07\xE7\x83a\x05\xF0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x08\xDFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t\xD0W\x805\x86\x81\x11\x15a\x08\xFBW`\0\x80\x81\xFD[\x87\x01`\xE0\x81\x8D\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81\x13\x15a\t1W`\0\x80\x81\xFD[a\t9a\x05~V[\x86\x83\x015\x89\x81\x11\x15a\tKW`\0\x80\x81\xFD[a\tY\x8F\x89\x83\x87\x01\x01a\x06\xC7V[\x82RPa\ti\x8E`@\x85\x01a\x07\x81V[\x87\x82\x01R`\xA0\x83\x015\x89\x81\x11\x15a\t\x80W`\0\x80\x81\xFD[a\t\x8E\x8F\x89\x83\x87\x01\x01a\x07\xC6V[`@\x83\x01RP`\xC0\x83\x015\x89\x81\x11\x15a\t\xA7W`\0\x80\x81\xFD[a\t\xB5\x8F\x89\x83\x87\x01\x01a\x069V[``\x83\x01RP\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x08\xE3V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\t\xE7W`\0\x80\xFD[Pa\t\xF4\x85\x82\x86\x01a\x069V[\x91PP\x92P\x92\x90PV[`\0\x80\x835\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x846\x03\x01\x81\x12a\n3W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nSW`\0\x80\xFD[\x806\x03\x82\x13\x15a\nbW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0` \x82\x84\x01\x01R`\0` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[`\0a\n\xBE\x82\x83a\t\xFEV[`@\x85Ra\n\xD0`@\x86\x01\x82\x84a\niV[\x91PPa\n\xE0` \x84\x01\x84a\t\xFEV[\x85\x83\x03` \x87\x01Ra\n\xF3\x83\x82\x84a\niV[\x96\x95PPPPPPV[`@\x80\x82R\x81\x01\x82\x90R`\0```\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83\x80[\x87\x81\x10\x15a\x0B\x9DW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x87\x86\x03\x01\x84R\x825\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x8A6\x03\x01\x81\x12a\x0B{W\x82\x83\xFD[a\x0B\x87\x86\x8B\x83\x01a\n\xB2V[\x95PP` \x93\x84\x01\x93\x92\x90\x92\x01\x91`\x01\x01a\x0B\x1BV[PPPP\x82\x81\x03` \x84\x01Ra\x0B\xB7\x81`\0\x81R` \x01\x90V[\x95\x94PPPPPV[`@\x81R`\0a\x0B\xD3`@\x83\x01\x84a\n\xB2V[\x82\x81\x03` \x93\x84\x01R`\0\x81R\x91\x90\x91\x01\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C+W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\x1FW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xD64\xC6\x7Ff\xA8\x1E\xAC\x0C\xB0Lu\xA1x\xD5h\x8By\xB1\x8Ej\x81\xCAO@\x1E\xDA8gAj\x85dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockFillContractWithOutputOverride<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockFillContractWithOutputOverride<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockFillContractWithOutputOverride<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockFillContractWithOutputOverride<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockFillContractWithOutputOverride<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockFillContractWithOutputOverride))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockFillContractWithOutputOverride<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_ABI.clone(),
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
                MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_ABI.clone(),
                MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_BYTECODE.clone().into(),
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
        ///Calls the contract's `reactorCallback` (0x585da628) function
        pub fn reactor_callback(
            &self,
            resolved_orders: ::std::vec::Vec<ResolvedOrder>,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 93, 166, 40], (resolved_orders, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOutputAmount` (0x24f9c434) function
        pub fn set_output_amount(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 249, 196, 52], amount)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockFillContractWithOutputOverride<M> {
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
        pub p1: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setOutputAmount` function with signature `setOutputAmount(uint256)` and selector `0x24f9c434`
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
    #[ethcall(name = "setOutputAmount", abi = "setOutputAmount(uint256)")]
    pub struct SetOutputAmountCall {
        pub amount: ::ethers::core::types::U256,
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
    pub enum MockFillContractWithOutputOverrideCalls {
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        ReactorCallback(ReactorCallbackCall),
        SetOutputAmount(SetOutputAmountCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFillContractWithOutputOverrideCalls {
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
            if let Ok(decoded) = <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReactorCallback(decoded));
            }
            if let Ok(decoded) = <SetOutputAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOutputAmount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFillContractWithOutputOverrideCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteBatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReactorCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOutputAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockFillContractWithOutputOverrideCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOutputAmount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for MockFillContractWithOutputOverrideCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteBatchCall>
    for MockFillContractWithOutputOverrideCalls {
        fn from(value: ExecuteBatchCall) -> Self {
            Self::ExecuteBatch(value)
        }
    }
    impl ::core::convert::From<ReactorCallbackCall>
    for MockFillContractWithOutputOverrideCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
    impl ::core::convert::From<SetOutputAmountCall>
    for MockFillContractWithOutputOverrideCalls {
        fn from(value: SetOutputAmountCall) -> Self {
            Self::SetOutputAmount(value)
        }
    }
}
