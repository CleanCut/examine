// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExamineRequest {
    /// The User-Agent header to examine.
    ///
    /// NOTE: In a non-toy project, this should probably be a separate ExamineInfo message which
    /// includes the user_agent field along with other relevant fields to examine.
    #[prost(string, tag="1")]
    pub user_agent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExamineResponse {
    /// Action to take based on the analysis of the information
    #[prost(enumeration="Action", tag="1")]
    pub action: i32,
}
/// Action recommended to take against an actor's traffic
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// Unused
    Unspecified = 0,
    /// Block the actor's traffic
    Block = 1,
    /// Allow the actor's traffic
    Allow = 2,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Unspecified => "ACTION_UNSPECIFIED",
            Action::Block => "ACTION_BLOCK",
            Action::Allow => "ACTION_ALLOW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "ACTION_BLOCK" => Some(Self::Block),
            "ACTION_ALLOW" => Some(Self::Allow),
            _ => None,
        }
    }
}
include!("examine.v1.serde.rs");
include!("examine.v1.tonic.rs");
// @@protoc_insertion_point(module)