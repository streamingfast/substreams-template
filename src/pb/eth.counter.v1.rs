#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counter {
    #[prost(uint64, tag="1")]
    pub number_of_transfers: u64,
}
