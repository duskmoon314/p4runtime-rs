#[doc = " The `Status` type defines a logical error model that is suitable for"]
#[doc = " different programming environments, including REST APIs and RPC APIs. It is"]
#[doc = " used by [gRPC](<https://github.com/grpc>). Each `Status` message contains"]
#[doc = " three pieces of data: error code, error message, and error details."]
#[doc = ""]
#[doc = " You can find out more about this error model and how to work with it in the"]
#[doc = " [API Design Guide](<https://cloud.google.com/apis/design/errors>)."]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Status {
    #[doc = " The status code, which should be an enum value of"]
    #[doc = " [google.rpc.Code][google.rpc.Code]."]
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[doc = " A developer-facing error message, which should be in English. Any"]
    #[doc = " user-facing error message should be localized and sent in the"]
    #[doc = " [google.rpc.Status.details][google.rpc.Status.details] field, or localized"]
    #[doc = " by the client."]
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[doc = " A list of messages that carry the error details.  There is a common set of"]
    #[doc = " message types for APIs to use."]
    #[prost(message, repeated, tag = "3")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
