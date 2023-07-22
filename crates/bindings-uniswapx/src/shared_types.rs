///`AllowanceTransferDetails(address,address,uint160,address)`
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
pub struct AllowanceTransferDetails {
    pub from: ::ethers::core::types::Address,
    pub to: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub token: ::ethers::core::types::Address,
}
///`PermitBatch((address,uint160,uint48,uint48)[],address,uint256)`
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
pub struct PermitBatch {
    pub details: ::std::vec::Vec<PermitDetails>,
    pub spender: ::ethers::core::types::Address,
    pub sig_deadline: ::ethers::core::types::U256,
}
///`PermitDetails(address,uint160,uint48,uint48)`
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
pub struct PermitDetails {
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub expiration: u64,
    pub nonce: u64,
}
///`PermitSingle((address,uint160,uint48,uint48),address,uint256)`
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
pub struct PermitSingle {
    pub details: PermitDetails,
    pub spender: ::ethers::core::types::Address,
    pub sig_deadline: ::ethers::core::types::U256,
}
///`TokenSpenderPair(address,address)`
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
pub struct TokenSpenderPair {
    pub token: ::ethers::core::types::Address,
    pub spender: ::ethers::core::types::Address,
}
///`PermitBatchTransferFrom((address,uint256)[],uint256,uint256)`
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
pub struct PermitBatchTransferFrom {
    pub permitted: ::std::vec::Vec<TokenPermissions>,
    pub nonce: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
}
///`PermitTransferFrom((address,uint256),uint256,uint256)`
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
pub struct PermitTransferFrom {
    pub permitted: TokenPermissions,
    pub nonce: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
}
///`SignatureTransferDetails(address,uint256)`
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
pub struct SignatureTransferDetails {
    pub to: ::ethers::core::types::Address,
    pub requested_amount: ::ethers::core::types::U256,
}
///`TokenPermissions(address,uint256)`
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
pub struct TokenPermissions {
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
}
///`ExactInputParams(bytes,address,uint256,uint256)`
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
pub struct ExactInputParams {
    pub path: ::ethers::core::types::Bytes,
    pub recipient: ::ethers::core::types::Address,
    pub amount_in: ::ethers::core::types::U256,
    pub amount_out_minimum: ::ethers::core::types::U256,
}
///`InputToken(address,uint256,uint256)`
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
pub struct InputToken {
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub max_amount: ::ethers::core::types::U256,
}
///`OrderInfo(address,address,uint256,uint256,address,bytes)`
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
pub struct OrderInfo {
    pub reactor: ::ethers::core::types::Address,
    pub swapper: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
    pub additional_validation_contract: ::ethers::core::types::Address,
    pub additional_validation_data: ::ethers::core::types::Bytes,
}
///`OutputToken(address,uint256,address)`
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
pub struct OutputToken {
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
}
///`ResolvedOrder((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)`
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
pub struct ResolvedOrder {
    pub info: OrderInfo,
    pub input: InputToken,
    pub outputs: ::std::vec::Vec<OutputToken>,
    pub sig: ::ethers::core::types::Bytes,
    pub hash: [u8; 32],
}
///`SignedOrder(bytes,bytes)`
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
pub struct SignedOrder {
    pub order: ::ethers::core::types::Bytes,
    pub sig: ::ethers::core::types::Bytes,
}
///`FuzzSelector(address,bytes4[])`
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
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
