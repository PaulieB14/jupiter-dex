#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountChange {
    #[prost(bytes = "vec", tag = "1")]
    pub pubkey: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub lamports: u64,
    #[prost(uint64, tag = "4")]
    pub slot: u64,
    #[prost(bool, tag = "5")]
    pub executable: bool,
    #[prost(uint64, tag = "6")]
    pub rent_epoch: u64,
    #[prost(bytes = "vec", tag = "7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "8")]
    pub deleted: bool,
}
#[derive(::prost::Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountChanges {
    #[prost(message, repeated, tag = "1")]
    pub changes: ::prost::alloc::vec::Vec<AccountChange>,
}
