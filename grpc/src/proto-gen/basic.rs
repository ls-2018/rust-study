// This file is @generated by prost-build.
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// created_at, last_visited_at, ..
    #[prost(map = "string, message", tag = "1")]
    pub timestamps: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        TimeQuery,
    >,
    #[prost(map = "string, message", tag = "2")]
    pub ids: ::std::collections::HashMap<::prost::alloc::string::String, IdQuery>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TimeQuery {
    #[prost(message, optional, tag = "1")]
    pub lower: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub upper: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdQuery {
    #[prost(uint32, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(sqlx::FromRow)]
#[derive(derive_builder::Builder)]
#[builder(setter(into, strip_option), default)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
