pub use i_swap_router_02::*;
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
pub mod i_swap_router_02 {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH9\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ExactInputParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"path\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInput\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ExactInputSingleParams\",\"name\":\"params\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMinimum\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint160\",\"name\":\"sqrtPriceLimitX96\",\"type\":\"uint160\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"exactInputSingle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapExactTokensForTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amountOut\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amountMinimum\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"unwrapWETH9\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ISWAPROUTER02_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ISwapRouter02<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISwapRouter02<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISwapRouter02<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISwapRouter02<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISwapRouter02<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ISwapRouter02)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISwapRouter02<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ISWAPROUTER02_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `WETH9` (0x4aa4a4fc) function
        pub fn weth9(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInput` (0xb858183f) function
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 88, 24, 63], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exactInputSingle` (0x04e45aaf) function
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 228, 90, 175], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0x5ae401dc) function
        pub fn multicall(
            &self,
            deadline: ::ethers::core::types::U256,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([90, 228, 1, 220], (deadline, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactTokensForTokens` (0x472b43f3) function
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 43, 67, 243], (amount_in, amount_out_min, path, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWETH9` (0x49616997) function
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 97, 105, 151], amount_minimum)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ISwapRouter02<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
    ///Container type for all input parameters for the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256))` and selector `0xb858183f`
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
    #[ethcall(name = "exactInput", abi = "exactInput((bytes,address,uint256,uint256))")]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    ///Container type for all input parameters for the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint160))` and selector `0x04e45aaf`
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
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(uint256,bytes[])` and selector `0x5ae401dc`
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
    #[ethcall(name = "multicall", abi = "multicall(uint256,bytes[])")]
    pub struct MulticallCall {
        pub deadline: ::ethers::core::types::U256,
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address)` and selector `0x472b43f3`
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
        name = "swapExactTokensForTokens",
        abi = "swapExactTokensForTokens(uint256,uint256,address[],address)"
    )]
    pub struct SwapExactTokensForTokensCall {
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unwrapWETH9` function with signature `unwrapWETH9(uint256)` and selector `0x49616997`
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
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISwapRouter02Calls {
        Weth9(Weth9Call),
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        Multicall(MulticallCall),
        SwapExactTokensForTokens(SwapExactTokensForTokensCall),
        UnwrapWETH9(UnwrapWETH9Call),
    }
    impl ::ethers::core::abi::AbiDecode for ISwapRouter02Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <Weth9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth9(decoded));
            }
            if let Ok(decoded)
                = <ExactInputCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExactInput(decoded));
            }
            if let Ok(decoded)
                = <ExactInputSingleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExactInputSingle(decoded));
            }
            if let Ok(decoded)
                = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded)
                = <SwapExactTokensForTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapExactTokensForTokens(decoded));
            }
            if let Ok(decoded)
                = <UnwrapWETH9Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnwrapWETH9(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISwapRouter02Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Weth9(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExactInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExactInputSingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactTokensForTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWETH9(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ISwapRouter02Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Weth9(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExactInputSingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactTokensForTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnwrapWETH9(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Weth9Call> for ISwapRouter02Calls {
        fn from(value: Weth9Call) -> Self {
            Self::Weth9(value)
        }
    }
    impl ::core::convert::From<ExactInputCall> for ISwapRouter02Calls {
        fn from(value: ExactInputCall) -> Self {
            Self::ExactInput(value)
        }
    }
    impl ::core::convert::From<ExactInputSingleCall> for ISwapRouter02Calls {
        fn from(value: ExactInputSingleCall) -> Self {
            Self::ExactInputSingle(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for ISwapRouter02Calls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<SwapExactTokensForTokensCall> for ISwapRouter02Calls {
        fn from(value: SwapExactTokensForTokensCall) -> Self {
            Self::SwapExactTokensForTokens(value)
        }
    }
    impl ::core::convert::From<UnwrapWETH9Call> for ISwapRouter02Calls {
        fn from(value: UnwrapWETH9Call) -> Self {
            Self::UnwrapWETH9(value)
        }
    }
    ///Container type for all return fields from the `WETH9` function with signature `WETH9()` and selector `0x4aa4a4fc`
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
    pub struct Weth9Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `exactInput` function with signature `exactInput((bytes,address,uint256,uint256))` and selector `0xb858183f`
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
    pub struct ExactInputReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `exactInputSingle` function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint160))` and selector `0x04e45aaf`
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
    pub struct ExactInputSingleReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `multicall` function with signature `multicall(uint256,bytes[])` and selector `0x5ae401dc`
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `swapExactTokensForTokens` function with signature `swapExactTokensForTokens(uint256,uint256,address[],address)` and selector `0x472b43f3`
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
    pub struct SwapExactTokensForTokensReturn {
        pub amount_out: ::ethers::core::types::U256,
    }
    ///`ExactInputSingleParams(address,address,uint24,address,uint256,uint256,uint160)`
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
    pub struct ExactInputSingleParams {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub fee: u32,
        pub recipient: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_minimum: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
}
