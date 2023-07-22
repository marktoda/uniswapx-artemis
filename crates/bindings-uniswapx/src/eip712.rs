pub use eip712::*;
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
pub mod eip712 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("eip712Domain"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("eip712Domain"),
                    inputs: ::std::vec![],
                    outputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("fields"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(1usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes1"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("chainId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("verifyingContract"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("salt"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("extensions"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },
                    ],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                },],
            )]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("EIP712DomainChanged"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("EIP712DomainChanged",),
                    inputs: ::std::vec![],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StringTooLong"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("StringTooLong"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("str"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
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
    pub static EIP712_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct EIP712<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EIP712<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EIP712<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EIP712<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EIP712<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EIP712))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EIP712<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EIP712_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EIP712DomainChanged` event
        pub fn eip712_domain_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, Eip712DomainChangedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, Eip712DomainChangedFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EIP712<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidShortString` with signature `InvalidShortString()` and selector `0xb3512b0c`
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
    #[etherror(name = "InvalidShortString", abi = "InvalidShortString()")]
    pub struct InvalidShortString;
    ///Custom Error type `StringTooLong` with signature `StringTooLong(string)` and selector `0x305a27a9`
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
    #[etherror(name = "StringTooLong", abi = "StringTooLong(string)")]
    pub struct StringTooLong {
        pub str: ::std::string::String,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EIP712Errors {
        InvalidShortString(InvalidShortString),
        StringTooLong(StringTooLong),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for EIP712Errors {
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
                <InvalidShortString as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidShortString(decoded));
            }
            if let Ok(decoded) = <StringTooLong as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StringTooLong(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EIP712Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidShortString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringTooLong(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for EIP712Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidShortString as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <StringTooLong as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for EIP712Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidShortString(element) => ::core::fmt::Display::fmt(element, f),
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for EIP712Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for EIP712Errors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for EIP712Errors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
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
    #[ethevent(name = "EIP712DomainChanged", abi = "EIP712DomainChanged()")]
    pub struct Eip712DomainChangedFilter;
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
    }
}
