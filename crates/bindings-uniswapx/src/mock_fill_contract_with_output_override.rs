pub use mock_fill_contract_with_output_override::*;
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
pub mod mock_fill_contract_with_output_override {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x08\xC6\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c$\xF9\xC44\x14a\0;W\x80c\x99C\xFA\x89\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02\xEEV[`\0UV[\0[a\0Na\0^6`\x04a\x06bV[`\0[\x83Q\x81\x10\x15a\x01:W`\0[\x84\x82\x81Q\x81\x10a\0\x7FWa\0\x7Fa\x08\x02V[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x01'W`\0\x85\x83\x81Q\x81\x10a\0\xA7Wa\0\xA7a\x08\x02V[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\0\xC4Wa\0\xC4a\x08\x02V[` \x02` \x01\x01Q\x90P`\0\x80T`\0\x14a\0\xE1W`\0Ta\0\xE7V[\x81` \x01Q[`@\x83\x01Q\x83Q\x91\x92Pa\x01\x12\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x83a\x01@V[PP\x80\x80a\x01\x1F\x90a\x081V[\x91PPa\0mV[P\x80a\x012\x81a\x081V[\x91PPa\0aV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x01\xF5W`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01\xB5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xBAV[``\x91P[PP\x90P\x80a\x01:W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x02\x1BV[PPPV[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x01:W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a\x03\0W`\0\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03YWa\x03Ya\x03\x07V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03YWa\x03Ya\x03\x07V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xC9Wa\x03\xC9a\x03\x07V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xEBWa\x03\xEBa\x03\x07V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x17W`\0\x80\xFD[PV[\x805a\x04%\x81a\x03\xF5V[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04;W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04UWa\x04Ua\x03\x07V[a\x04\x86` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x03\x82V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\x9BW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x04\xCAW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04\xEEWa\x04\xEEa\x03\x07V[\x81`@R\x82\x93P\x845\x91Pa\x05\x02\x82a\x03\xF5V[\x90\x82R` \x84\x015\x90a\x05\x14\x82a\x03\xF5V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x05>\x82a\x03\xF5V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x05XW`\0\x80\xFD[Pa\x05e\x85\x82\x86\x01a\x04*V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x05\x84W`\0\x80\xFD[a\x05\x8Ca\x036V[\x90P\x815a\x05\x99\x81a\x03\xF5V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x05\xC8W`\0\x80\xFD[\x815` a\x05\xDDa\x05\xD8\x83a\x03\xD1V[a\x03\x82V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05\xFCW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06UW\x81\x81\x8A\x03\x12\x15a\x06\x18W`\0\x80\x81\xFD[a\x06 a\x036V[\x815a\x06+\x81a\x03\xF5V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x06D\x81a\x03\xF5V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x06\0V[P\x90\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06wW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\x8FW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x06\xA3W`\0\x80\xFD[\x815` a\x06\xB3a\x05\xD8\x83a\x03\xD1V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x06\xD2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x07\xC3W\x805\x86\x81\x11\x15a\x06\xEDW`\0\x80\xFD[\x87\x01`\xE0\x81\x8E\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x12\x15a\x07!W`\0\x80\xFD[a\x07)a\x03_V[\x85\x82\x015\x88\x81\x11\x15a\x07:W`\0\x80\xFD[a\x07H\x8F\x88\x83\x86\x01\x01a\x04\xB8V[\x82RPa\x07X\x8E`@\x84\x01a\x05rV[\x86\x82\x01R`\xA0\x82\x015\x88\x81\x11\x15a\x07oW`\0\x80\x81\xFD[a\x07}\x8F\x88\x83\x86\x01\x01a\x05\xB7V[`@\x83\x01RP`\xC0\x82\x015\x88\x81\x11\x15a\x07\x96W`\0\x80\x81\xFD[a\x07\xA4\x8F\x88\x83\x86\x01\x01a\x04*V[``\x83\x01RP`\xE0\x91\x90\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x06\xD6V[P\x97Pa\x07\xD3\x90P\x88\x82\x01a\x04\x1AV[\x95PPP`@\x86\x015\x91P\x80\x82\x11\x15a\x07\xEBW`\0\x80\xFD[Pa\x07\xF8\x86\x82\x87\x01a\x04*V[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x08\x89W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xA0\xED\x15-q/$\x19\x8E{mR\x1F\xEA\x13\xE4\xAFY\xA9\xAF\xDC\x0E\x86;\x1Bx\xB0\xE3\x0C\xFCmudsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c$\xF9\xC44\x14a\0;W\x80c\x99C\xFA\x89\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02\xEEV[`\0UV[\0[a\0Na\0^6`\x04a\x06bV[`\0[\x83Q\x81\x10\x15a\x01:W`\0[\x84\x82\x81Q\x81\x10a\0\x7FWa\0\x7Fa\x08\x02V[` \x02` \x01\x01Q`@\x01QQ\x81\x10\x15a\x01'W`\0\x85\x83\x81Q\x81\x10a\0\xA7Wa\0\xA7a\x08\x02V[` \x02` \x01\x01Q`@\x01Q\x82\x81Q\x81\x10a\0\xC4Wa\0\xC4a\x08\x02V[` \x02` \x01\x01Q\x90P`\0\x80T`\0\x14a\0\xE1W`\0Ta\0\xE7V[\x81` \x01Q[`@\x83\x01Q\x83Q\x91\x92Pa\x01\x12\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x83a\x01@V[PP\x80\x80a\x01\x1F\x90a\x081V[\x91PPa\0mV[P\x80a\x012\x81a\x081V[\x91PPa\0aV[PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16a\x01\xF5W`\0\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01\xB5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xBAV[``\x91P[PP\x90P\x80a\x01:W`@Q\x7F\xF4\xB3\xB1\xBC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x02\x1BV[PPPV[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x01:W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0` \x82\x84\x03\x12\x15a\x03\0W`\0\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03YWa\x03Ya\x03\x07V[`@R\x90V[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03YWa\x03Ya\x03\x07V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xC9Wa\x03\xC9a\x03\x07V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\xEBWa\x03\xEBa\x03\x07V[P`\x05\x1B` \x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\x17W`\0\x80\xFD[PV[\x805a\x04%\x81a\x03\xF5V[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04;W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04UWa\x04Ua\x03\x07V[a\x04\x86` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a\x03\x82V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\x9BW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\xC0\x82\x84\x03\x12\x15a\x04\xCAW`\0\x80\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x04\xEEWa\x04\xEEa\x03\x07V[\x81`@R\x82\x93P\x845\x91Pa\x05\x02\x82a\x03\xF5V[\x90\x82R` \x84\x015\x90a\x05\x14\x82a\x03\xF5V[\x81` \x84\x01R`@\x85\x015`@\x84\x01R``\x85\x015``\x84\x01R`\x80\x85\x015\x91Pa\x05>\x82a\x03\xF5V[\x81`\x80\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x05XW`\0\x80\xFD[Pa\x05e\x85\x82\x86\x01a\x04*V[`\xA0\x83\x01RPP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\x05\x84W`\0\x80\xFD[a\x05\x8Ca\x036V[\x90P\x815a\x05\x99\x81a\x03\xF5V[\x80\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x05\xC8W`\0\x80\xFD[\x815` a\x05\xDDa\x05\xD8\x83a\x03\xD1V[a\x03\x82V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05\xFCW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06UW\x81\x81\x8A\x03\x12\x15a\x06\x18W`\0\x80\x81\xFD[a\x06 a\x036V[\x815a\x06+\x81a\x03\xF5V[\x81R\x81\x86\x015\x86\x82\x01R`@\x80\x83\x015a\x06D\x81a\x03\xF5V[\x90\x82\x01R\x84R\x92\x84\x01\x92\x81\x01a\x06\0V[P\x90\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06wW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\x8FW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x06\xA3W`\0\x80\xFD[\x815` a\x06\xB3a\x05\xD8\x83a\x03\xD1V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x06\xD2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x07\xC3W\x805\x86\x81\x11\x15a\x06\xEDW`\0\x80\xFD[\x87\x01`\xE0\x81\x8E\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x12\x15a\x07!W`\0\x80\xFD[a\x07)a\x03_V[\x85\x82\x015\x88\x81\x11\x15a\x07:W`\0\x80\xFD[a\x07H\x8F\x88\x83\x86\x01\x01a\x04\xB8V[\x82RPa\x07X\x8E`@\x84\x01a\x05rV[\x86\x82\x01R`\xA0\x82\x015\x88\x81\x11\x15a\x07oW`\0\x80\x81\xFD[a\x07}\x8F\x88\x83\x86\x01\x01a\x05\xB7V[`@\x83\x01RP`\xC0\x82\x015\x88\x81\x11\x15a\x07\x96W`\0\x80\x81\xFD[a\x07\xA4\x8F\x88\x83\x86\x01\x01a\x04*V[``\x83\x01RP`\xE0\x91\x90\x91\x015`\x80\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\x06\xD6V[P\x97Pa\x07\xD3\x90P\x88\x82\x01a\x04\x1AV[\x95PPP`@\x86\x015\x91P\x80\x82\x11\x15a\x07\xEBW`\0\x80\xFD[Pa\x07\xF8\x86\x82\x87\x01a\x04*V[\x91PP\x92P\x92P\x92V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[`\0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x08\x89W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xA0\xED\x15-q/$\x19\x8E{mR\x1F\xEA\x13\xE4\xAFY\xA9\xAF\xDC\x0E\x86;\x1Bx\xB0\xE3\x0C\xFCmudsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_ABI.clone(),
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
                MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_ABI.clone(),
                MOCKFILLCONTRACTWITHOUTPUTOVERRIDE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `reactorCallback` (0x9943fa89) function
        pub fn reactor_callback(
            &self,
            resolved_orders: ::std::vec::Vec<ResolvedOrder>,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 67, 250, 137], (resolved_orders, p1, p2))
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
        for MockFillContractWithOutputOverride<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NativeTransferFailed` with signature `NativeTransferFailed()` and selector `0xf4b3b1bc`
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
    #[etherror(name = "NativeTransferFailed", abi = "NativeTransferFailed()")]
    pub struct NativeTransferFailed;
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
        pub p1: ::ethers::core::types::Address,
        pub p2: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setOutputAmount` function with signature `setOutputAmount(uint256)` and selector `0x24f9c434`
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
    #[ethcall(name = "setOutputAmount", abi = "setOutputAmount(uint256)")]
    pub struct SetOutputAmountCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MockFillContractWithOutputOverrideCalls {
        ReactorCallback(ReactorCallbackCall),
        SetOutputAmount(SetOutputAmountCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockFillContractWithOutputOverrideCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ReactorCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReactorCallback(decoded));
            }
            if let Ok(decoded) =
                <SetOutputAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOutputAmount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockFillContractWithOutputOverrideCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ReactorCallback(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOutputAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockFillContractWithOutputOverrideCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ReactorCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOutputAmount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ReactorCallbackCall> for MockFillContractWithOutputOverrideCalls {
        fn from(value: ReactorCallbackCall) -> Self {
            Self::ReactorCallback(value)
        }
    }
    impl ::core::convert::From<SetOutputAmountCall> for MockFillContractWithOutputOverrideCalls {
        fn from(value: SetOutputAmountCall) -> Self {
            Self::SetOutputAmount(value)
        }
    }
}
