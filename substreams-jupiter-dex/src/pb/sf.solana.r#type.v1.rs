/// @derive(::prost::Message)
#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmedTransaction {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(bytes = "vec", tag = "2")]
    pub slot: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub block_time: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub meta: ::prost::alloc::vec::Vec<u8>,
}
/// @derive(::prost::Message)
#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes = "vec", tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<Message>,
}
/// @derive(::prost::Message)
#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub account_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", tag = "2")]
    pub recent_blockhash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub instructions: ::prost::alloc::vec::Vec<CompiledInstruction>,
}
/// @derive(::prost::Message)
#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompiledInstruction {
    #[prost(uint32, tag = "1")]
    pub program_id_index: u32,
    #[prost(uint32, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// @derive(::prost::Message)
#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
    #[prost(int64, tag = "2")]
    pub slot: i64,
    #[prost(message, optional, tag = "3")]
    pub block_time: ::core::option::Option<BlockTime>,
    #[prost(bytes = "vec", tag = "4")]
    pub blockhash: ::prost::alloc::vec::Vec<u8>,
}
/// @derive(::prost::Message)
#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockTime {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
}
