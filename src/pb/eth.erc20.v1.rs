#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub balance_change_from: ::prost::alloc::vec::Vec<BalanceChange>,
    #[prost(message, repeated, tag="5")]
    pub balance_change_to: ::prost::alloc::vec::Vec<BalanceChange>,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    #[prost(string, tag="1")]
    pub old_balance: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_balance: ::prost::alloc::string::String,
}
