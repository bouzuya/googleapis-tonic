// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParsedExpr {
    #[prost(message, optional, tag = "2")]
    pub expr: ::core::option::Option<Expr>,
    #[prost(message, optional, tag = "3")]
    pub source_info: ::core::option::Option<SourceInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expr {
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(oneof = "expr::ExprKind", tags = "3, 4, 5, 6, 7, 8, 9")]
    pub expr_kind: ::core::option::Option<expr::ExprKind>,
}
/// Nested message and enum types in `Expr`.
pub mod expr {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ident {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Select {
        #[prost(message, optional, boxed, tag = "1")]
        pub operand: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        #[prost(string, tag = "2")]
        pub field: ::prost::alloc::string::String,
        #[prost(bool, tag = "3")]
        pub test_only: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Call {
        #[prost(message, optional, boxed, tag = "1")]
        pub target: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        #[prost(string, tag = "2")]
        pub function: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub args: ::prost::alloc::vec::Vec<super::Expr>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateList {
        #[prost(message, repeated, tag = "1")]
        pub elements: ::prost::alloc::vec::Vec<super::Expr>,
        #[prost(int32, repeated, tag = "2")]
        pub optional_indices: ::prost::alloc::vec::Vec<i32>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CreateStruct {
        #[prost(string, tag = "1")]
        pub message_name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub entries: ::prost::alloc::vec::Vec<create_struct::Entry>,
    }
    /// Nested message and enum types in `CreateStruct`.
    pub mod create_struct {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Entry {
            #[prost(int64, tag = "1")]
            pub id: i64,
            #[prost(message, optional, tag = "4")]
            pub value: ::core::option::Option<super::super::Expr>,
            #[prost(bool, tag = "5")]
            pub optional_entry: bool,
            #[prost(oneof = "entry::KeyKind", tags = "2, 3")]
            pub key_kind: ::core::option::Option<entry::KeyKind>,
        }
        /// Nested message and enum types in `Entry`.
        pub mod entry {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum KeyKind {
                #[prost(string, tag = "2")]
                FieldKey(::prost::alloc::string::String),
                #[prost(message, tag = "3")]
                MapKey(super::super::super::Expr),
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Comprehension {
        #[prost(string, tag = "1")]
        pub iter_var: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub iter_var2: ::prost::alloc::string::String,
        #[prost(message, optional, boxed, tag = "2")]
        pub iter_range: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        #[prost(string, tag = "3")]
        pub accu_var: ::prost::alloc::string::String,
        #[prost(message, optional, boxed, tag = "4")]
        pub accu_init: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        #[prost(message, optional, boxed, tag = "5")]
        pub loop_condition: ::core::option::Option<
            ::prost::alloc::boxed::Box<super::Expr>,
        >,
        #[prost(message, optional, boxed, tag = "6")]
        pub loop_step: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
        #[prost(message, optional, boxed, tag = "7")]
        pub result: ::core::option::Option<::prost::alloc::boxed::Box<super::Expr>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExprKind {
        #[prost(message, tag = "3")]
        ConstExpr(super::Constant),
        #[prost(message, tag = "4")]
        IdentExpr(Ident),
        #[prost(message, tag = "5")]
        SelectExpr(::prost::alloc::boxed::Box<Select>),
        #[prost(message, tag = "6")]
        CallExpr(::prost::alloc::boxed::Box<Call>),
        #[prost(message, tag = "7")]
        ListExpr(CreateList),
        #[prost(message, tag = "8")]
        StructExpr(CreateStruct),
        #[prost(message, tag = "9")]
        ComprehensionExpr(::prost::alloc::boxed::Box<Comprehension>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constant {
    #[prost(oneof = "constant::ConstantKind", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub constant_kind: ::core::option::Option<constant::ConstantKind>,
}
/// Nested message and enum types in `Constant`.
pub mod constant {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConstantKind {
        #[prost(enumeration = "::prost_types::NullValue", tag = "1")]
        NullValue(i32),
        #[prost(bool, tag = "2")]
        BoolValue(bool),
        #[prost(int64, tag = "3")]
        Int64Value(i64),
        #[prost(uint64, tag = "4")]
        Uint64Value(u64),
        #[prost(double, tag = "5")]
        DoubleValue(f64),
        #[prost(string, tag = "6")]
        StringValue(::prost::alloc::string::String),
        #[prost(bytes, tag = "7")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "8")]
        DurationValue(::prost_types::Duration),
        #[prost(message, tag = "9")]
        TimestampValue(::prost_types::Timestamp),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceInfo {
    #[prost(string, tag = "1")]
    pub syntax_version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub location: ::prost::alloc::string::String,
    #[prost(int32, repeated, tag = "3")]
    pub line_offsets: ::prost::alloc::vec::Vec<i32>,
    #[prost(btree_map = "int64, int32", tag = "4")]
    pub positions: ::prost::alloc::collections::BTreeMap<i64, i32>,
    #[prost(btree_map = "int64, message", tag = "5")]
    pub macro_calls: ::prost::alloc::collections::BTreeMap<i64, Expr>,
    #[prost(message, repeated, tag = "6")]
    pub extensions: ::prost::alloc::vec::Vec<source_info::Extension>,
}
/// Nested message and enum types in `SourceInfo`.
pub mod source_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Extension {
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        #[prost(enumeration = "extension::Component", repeated, tag = "2")]
        pub affected_components: ::prost::alloc::vec::Vec<i32>,
        #[prost(message, optional, tag = "3")]
        pub version: ::core::option::Option<extension::Version>,
    }
    /// Nested message and enum types in `Extension`.
    pub mod extension {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct Version {
            #[prost(int64, tag = "1")]
            pub major: i64,
            #[prost(int64, tag = "2")]
            pub minor: i64,
        }
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Component {
            Unspecified = 0,
            Parser = 1,
            TypeChecker = 2,
            Runtime = 3,
        }
        impl Component {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Component::Unspecified => "COMPONENT_UNSPECIFIED",
                    Component::Parser => "COMPONENT_PARSER",
                    Component::TypeChecker => "COMPONENT_TYPE_CHECKER",
                    Component::Runtime => "COMPONENT_RUNTIME",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "COMPONENT_UNSPECIFIED" => Some(Self::Unspecified),
                    "COMPONENT_PARSER" => Some(Self::Parser),
                    "COMPONENT_TYPE_CHECKER" => Some(Self::TypeChecker),
                    "COMPONENT_RUNTIME" => Some(Self::Runtime),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourcePosition {
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub offset: i32,
    #[prost(int32, tag = "3")]
    pub line: i32,
    #[prost(int32, tag = "4")]
    pub column: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckedExpr {
    #[prost(btree_map = "int64, message", tag = "2")]
    pub reference_map: ::prost::alloc::collections::BTreeMap<i64, Reference>,
    #[prost(btree_map = "int64, message", tag = "3")]
    pub type_map: ::prost::alloc::collections::BTreeMap<i64, Type>,
    #[prost(message, optional, tag = "5")]
    pub source_info: ::core::option::Option<SourceInfo>,
    #[prost(string, tag = "6")]
    pub expr_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub expr: ::core::option::Option<Expr>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    #[prost(
        oneof = "r#type::TypeKind",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14"
    )]
    pub type_kind: ::core::option::Option<r#type::TypeKind>,
}
/// Nested message and enum types in `Type`.
pub mod r#type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListType {
        #[prost(message, optional, boxed, tag = "1")]
        pub elem_type: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MapType {
        #[prost(message, optional, boxed, tag = "1")]
        pub key_type: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
        #[prost(message, optional, boxed, tag = "2")]
        pub value_type: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FunctionType {
        #[prost(message, optional, boxed, tag = "1")]
        pub result_type: ::core::option::Option<::prost::alloc::boxed::Box<super::Type>>,
        #[prost(message, repeated, tag = "2")]
        pub arg_types: ::prost::alloc::vec::Vec<super::Type>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AbstractType {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub parameter_types: ::prost::alloc::vec::Vec<super::Type>,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PrimitiveType {
        Unspecified = 0,
        Bool = 1,
        Int64 = 2,
        Uint64 = 3,
        Double = 4,
        String = 5,
        Bytes = 6,
    }
    impl PrimitiveType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PrimitiveType::Unspecified => "PRIMITIVE_TYPE_UNSPECIFIED",
                PrimitiveType::Bool => "BOOL",
                PrimitiveType::Int64 => "INT64",
                PrimitiveType::Uint64 => "UINT64",
                PrimitiveType::Double => "DOUBLE",
                PrimitiveType::String => "STRING",
                PrimitiveType::Bytes => "BYTES",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PRIMITIVE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BOOL" => Some(Self::Bool),
                "INT64" => Some(Self::Int64),
                "UINT64" => Some(Self::Uint64),
                "DOUBLE" => Some(Self::Double),
                "STRING" => Some(Self::String),
                "BYTES" => Some(Self::Bytes),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum WellKnownType {
        Unspecified = 0,
        Any = 1,
        Timestamp = 2,
        Duration = 3,
    }
    impl WellKnownType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WellKnownType::Unspecified => "WELL_KNOWN_TYPE_UNSPECIFIED",
                WellKnownType::Any => "ANY",
                WellKnownType::Timestamp => "TIMESTAMP",
                WellKnownType::Duration => "DURATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WELL_KNOWN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ANY" => Some(Self::Any),
                "TIMESTAMP" => Some(Self::Timestamp),
                "DURATION" => Some(Self::Duration),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeKind {
        #[prost(message, tag = "1")]
        Dyn(()),
        #[prost(enumeration = "::prost_types::NullValue", tag = "2")]
        Null(i32),
        #[prost(enumeration = "PrimitiveType", tag = "3")]
        Primitive(i32),
        #[prost(enumeration = "PrimitiveType", tag = "4")]
        Wrapper(i32),
        #[prost(enumeration = "WellKnownType", tag = "5")]
        WellKnown(i32),
        #[prost(message, tag = "6")]
        ListType(::prost::alloc::boxed::Box<ListType>),
        #[prost(message, tag = "7")]
        MapType(::prost::alloc::boxed::Box<MapType>),
        #[prost(message, tag = "8")]
        Function(::prost::alloc::boxed::Box<FunctionType>),
        #[prost(string, tag = "9")]
        MessageType(::prost::alloc::string::String),
        #[prost(string, tag = "10")]
        TypeParam(::prost::alloc::string::String),
        #[prost(message, tag = "11")]
        Type(::prost::alloc::boxed::Box<super::Type>),
        #[prost(message, tag = "12")]
        Error(()),
        #[prost(message, tag = "14")]
        AbstractType(AbstractType),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decl {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "decl::DeclKind", tags = "2, 3")]
    pub decl_kind: ::core::option::Option<decl::DeclKind>,
}
/// Nested message and enum types in `Decl`.
pub mod decl {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentDecl {
        #[prost(message, optional, tag = "1")]
        pub r#type: ::core::option::Option<super::Type>,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<super::Constant>,
        #[prost(string, tag = "3")]
        pub doc: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FunctionDecl {
        #[prost(message, repeated, tag = "1")]
        pub overloads: ::prost::alloc::vec::Vec<function_decl::Overload>,
    }
    /// Nested message and enum types in `FunctionDecl`.
    pub mod function_decl {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Overload {
            #[prost(string, tag = "1")]
            pub overload_id: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "2")]
            pub params: ::prost::alloc::vec::Vec<super::super::Type>,
            #[prost(string, repeated, tag = "3")]
            pub type_params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            #[prost(message, optional, tag = "4")]
            pub result_type: ::core::option::Option<super::super::Type>,
            #[prost(bool, tag = "5")]
            pub is_instance_function: bool,
            #[prost(string, tag = "6")]
            pub doc: ::prost::alloc::string::String,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeclKind {
        #[prost(message, tag = "2")]
        Ident(IdentDecl),
        #[prost(message, tag = "3")]
        Function(FunctionDecl),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reference {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub overload_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub value: ::core::option::Option<Constant>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Kind", tags = "1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 15")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(enumeration = "::prost_types::NullValue", tag = "1")]
        NullValue(i32),
        #[prost(bool, tag = "2")]
        BoolValue(bool),
        #[prost(int64, tag = "3")]
        Int64Value(i64),
        #[prost(uint64, tag = "4")]
        Uint64Value(u64),
        #[prost(double, tag = "5")]
        DoubleValue(f64),
        #[prost(string, tag = "6")]
        StringValue(::prost::alloc::string::String),
        #[prost(bytes, tag = "7")]
        BytesValue(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "9")]
        EnumValue(super::EnumValue),
        #[prost(message, tag = "10")]
        ObjectValue(::prost_types::Any),
        #[prost(message, tag = "11")]
        MapValue(super::MapValue),
        #[prost(message, tag = "12")]
        ListValue(super::ListValue),
        #[prost(string, tag = "15")]
        TypeValue(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumValue {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapValue {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<map_value::Entry>,
}
/// Nested message and enum types in `MapValue`.
pub mod map_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(message, optional, tag = "1")]
        pub key: ::core::option::Option<super::Value>,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<super::Value>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Explain {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    #[prost(message, repeated, tag = "2")]
    pub expr_steps: ::prost::alloc::vec::Vec<explain::ExprStep>,
}
/// Nested message and enum types in `Explain`.
pub mod explain {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct ExprStep {
        #[prost(int64, tag = "1")]
        pub id: i64,
        #[prost(int32, tag = "2")]
        pub value_index: i32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvalState {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<ExprValue>,
    #[prost(message, repeated, tag = "3")]
    pub results: ::prost::alloc::vec::Vec<eval_state::Result>,
}
/// Nested message and enum types in `EvalState`.
pub mod eval_state {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(int64, tag = "1")]
        pub expr: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExprValue {
    #[prost(oneof = "expr_value::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<expr_value::Kind>,
}
/// Nested message and enum types in `ExprValue`.
pub mod expr_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Value(super::Value),
        #[prost(message, tag = "2")]
        Error(super::ErrorSet),
        #[prost(message, tag = "3")]
        Unknown(super::UnknownSet),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorSet {
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnknownSet {
    #[prost(int64, repeated, tag = "1")]
    pub exprs: ::prost::alloc::vec::Vec<i64>,
}