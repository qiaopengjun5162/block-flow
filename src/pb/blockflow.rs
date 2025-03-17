// This file is @generated by prost-build.
#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub previous_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    #[prost(int64, tag = "5")]
    pub nonce: i64,
}
#[derive(Hash, Eq, serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct BlockResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<Block>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
