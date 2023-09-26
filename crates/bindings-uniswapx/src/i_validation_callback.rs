pub use i_validation_callback::*;
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
pub mod i_validation_callback {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"filler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct ResolvedOrder\",\"name\":\"resolvedOrder\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct OrderInfo\",\"name\":\"info\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract IReactor\",\"name\":\"reactor\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"swapper\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"contract IValidationCallback\",\"name\":\"additionalValidationContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"additionalValidationData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct InputToken\",\"name\":\"input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract ERC20\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct OutputToken[]\",\"name\":\"outputs\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"validate\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IVALIDATIONCALLBACK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IValidationCallback<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IValidationCallback<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IValidationCallback<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IValidationCallback<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IValidationCallback<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IValidationCallback))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IValidationCallback<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IVALIDATIONCALLBACK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `validate` (0x6e84ba2b) function
        pub fn validate(
            &self,
            filler: ::ethers::core::types::Address,
            resolved_order: ResolvedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 132, 186, 43], (filler, resolved_order))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IValidationCallback<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `validate` function with signature `validate(address,((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))` and selector `0x6e84ba2b`
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
    #[ethcall(
        name = "validate",
        abi = "validate(address,((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32))"
    )]
    pub struct ValidateCall {
        pub filler: ::ethers::core::types::Address,
        pub resolved_order: ResolvedOrder,
    }
}
