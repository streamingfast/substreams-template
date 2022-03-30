#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contracts {
    #[prost(message, repeated, tag="1")]
    pub contracts: ::std::vec::Vec<Contract>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(bytes, tag="1")]
    pub address: std::vec::Vec<u8>,
}
