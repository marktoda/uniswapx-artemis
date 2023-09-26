pub use ierc5267::*;
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
pub mod ierc5267 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"event\",\"name\":\"EIP712DomainChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"eip712Domain\",\"outputs\":[{\"internalType\":\"bytes1\",\"name\":\"fields\",\"type\":\"bytes1\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"chainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"verifyingContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"extensions\",\"type\":\"uint256[]\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC5267_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IERC5267<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC5267<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC5267<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC5267<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC5267<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC5267)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC5267<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC5267_ABI.clone(),
                    client,
                ),
            )
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC5267<M> {
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
        Hash
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
        Hash
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
        Hash
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
