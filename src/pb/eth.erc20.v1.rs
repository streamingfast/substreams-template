#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::std::vec::Vec<Transfer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub from: std::string::String,
    #[prost(string, tag="2")]
    pub to: std::string::String,
    #[prost(string, tag="3")]
    pub amount: std::string::String,
    #[prost(message, repeated, tag="4")]
    pub balance_change_from: ::std::vec::Vec<BalanceChange>,
    #[prost(message, repeated, tag="5")]
    pub balance_change_to: ::std::vec::Vec<BalanceChange>,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChange {
    #[prost(string, tag="1")]
    pub old_balance: std::string::String,
    #[prost(string, tag="2")]
    pub new_balance: std::string::String,
}
