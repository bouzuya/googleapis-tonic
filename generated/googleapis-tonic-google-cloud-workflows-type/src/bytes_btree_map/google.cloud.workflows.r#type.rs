// This file is @generated by prost-build.
/// Logged during a workflow execution if the customer has requested call
/// logging.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EngineCallLog {
    /// The execution ID of the execution where the call occurred.
    #[prost(string, tag = "1")]
    pub execution_id: ::prost::alloc::string::String,
    /// The point in time when the activity occurred.
    #[prost(message, optional, tag = "2")]
    pub activity_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The state of the function execution.
    #[prost(enumeration = "engine_call_log::State", tag = "3")]
    pub state: i32,
    /// The name of the step in which the call took place, truncated if necessary.
    #[prost(string, tag = "4")]
    pub step: ::prost::alloc::string::String,
    /// The name of the target of the call, truncated if necessary.
    #[prost(string, tag = "5")]
    pub callee: ::prost::alloc::string::String,
    #[prost(oneof = "engine_call_log::Details", tags = "6, 7, 8, 9")]
    pub details: ::core::option::Option<engine_call_log::Details>,
}
/// Nested message and enum types in `EngineCallLog`.
pub mod engine_call_log {
    /// Information about an argument to a called function.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallArg {
        /// A function argument, serialized to a string. This may be truncated for
        /// size reasons.
        #[prost(string, tag = "1")]
        pub argument: ::prost::alloc::string::String,
    }
    /// Information about the start of a call.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Begun {
        /// The arguments passed to the function. Only one of 'args' and 'named_args'
        /// will be populated.
        #[prost(message, repeated, tag = "1")]
        pub args: ::prost::alloc::vec::Vec<CallArg>,
        /// The arguments passed to the function, as a map with the argument names as
        /// the keys. The values may be JSON values or they may be the serialized
        /// string forms of the arguments truncated for size reasons. Only one of
        /// 'args' and 'named_args' will be populated.
        #[prost(btree_map = "string, message", tag = "2")]
        pub named_args: ::prost::alloc::collections::BTreeMap<
            ::prost::alloc::string::String,
            ::prost_types::Value,
        >,
    }
    /// Information about the end of a successful call.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Succeeded {
        /// The time when the call started.
        #[prost(message, optional, tag = "1")]
        pub call_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The result of the call, if the call succeeded, serialized to a string.
        /// This may be truncated for size reasons.
        #[prost(string, tag = "2")]
        pub response: ::prost::alloc::string::String,
    }
    /// Information about the end of a failed call.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExceptionRaised {
        /// The time when the call started.
        #[prost(message, optional, tag = "1")]
        pub call_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The exception message which terminated the call, truncated if necessary.
        #[prost(string, tag = "2")]
        pub exception: ::prost::alloc::string::String,
        /// The name of the step where the failure originates, if known. Truncated
        /// if necessary.
        #[prost(string, tag = "3")]
        pub origin: ::prost::alloc::string::String,
    }
    /// Information about an exception which was handled.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExceptionHandled {
        /// The time when the call started.
        #[prost(message, optional, tag = "1")]
        pub call_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The exception message which was handled, truncated if necessary.
        #[prost(string, tag = "2")]
        pub exception: ::prost::alloc::string::String,
        /// The name of the step where the failure originates, if known. Truncated
        /// if necessary.
        #[prost(string, tag = "3")]
        pub origin: ::prost::alloc::string::String,
    }
    /// The state of a function call.
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
    pub enum State {
        /// Function call state is unspecified or unknown.
        Unspecified = 0,
        /// Function call is starting.
        Begun = 1,
        /// Function call has completed successfully.
        Succeeded = 2,
        /// Function call did not succeed because an exception was raised.
        ExceptionRaised = 3,
        /// Function call handled an exception and is continuing.
        ExceptionHandled = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Begun => "BEGUN",
                Self::Succeeded => "SUCCEEDED",
                Self::ExceptionRaised => "EXCEPTION_RAISED",
                Self::ExceptionHandled => "EXCEPTION_HANDLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "BEGUN" => Some(Self::Begun),
                "SUCCEEDED" => Some(Self::Succeeded),
                "EXCEPTION_RAISED" => Some(Self::ExceptionRaised),
                "EXCEPTION_HANDLED" => Some(Self::ExceptionHandled),
                _ => None,
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Appears at the start of a call.
        #[prost(message, tag = "6")]
        Begun(Begun),
        /// Appears when a call returns successfully.
        #[prost(message, tag = "7")]
        Succeeded(Succeeded),
        /// Appears when a call returns because an exception was raised.
        #[prost(message, tag = "8")]
        ExceptionRaised(ExceptionRaised),
        /// Appears when an exception is handled and normal execution resumes.
        #[prost(message, tag = "9")]
        ExceptionHandled(ExceptionHandled),
    }
}
/// Logged during the lifetime of Workflow Execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionsSystemLog {
    /// Human readable contents of the log in English. The size limit is 5 kB.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The absolute point in time when the activity happened.
    #[prost(message, optional, tag = "2")]
    pub activity_time: ::core::option::Option<::prost_types::Timestamp>,
    /// State of the execution when the log was created.
    #[prost(enumeration = "executions_system_log::State", tag = "3")]
    pub state: i32,
    /// Detailed log information.
    #[prost(oneof = "executions_system_log::Details", tags = "4, 5, 6")]
    pub details: ::core::option::Option<executions_system_log::Details>,
}
/// Nested message and enum types in `ExecutionsSystemLog`.
pub mod executions_system_log {
    /// Detailed information about the start of the execution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Start {
        /// The execution input argument.
        #[prost(string, tag = "2")]
        pub argument: ::prost::alloc::string::String,
    }
    /// Detailed information about the successful finish of the execution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Success {
        /// The final result of the execution.
        #[prost(string, tag = "2")]
        pub result: ::prost::alloc::string::String,
    }
    /// Detailed information about the execution failure.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Failure {
        /// The exception message, e.g. "division by zero". The size limit is 1 kB.
        #[prost(string, tag = "1")]
        pub exception: ::prost::alloc::string::String,
        /// The code location of the statement that has created the log. For example,
        /// a log created in subworkflow 'Foo' in step 'bar' will have its source
        /// equal to 'Foo.bar'. The size limit is 1 kB.
        #[prost(string, tag = "2")]
        pub source: ::prost::alloc::string::String,
    }
    /// Possible states of the execution. There could be more states in the future.
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
    pub enum State {
        /// Invalid state.
        Unspecified = 0,
        /// The Workflow Execution is in progress.
        Active = 1,
        /// The Workflow Execution has finished successfully.
        Succeeded = 2,
        /// The Workflow Execution failed with an error.
        Failed = 3,
        /// The Workflow Execution has been stopped intentionally.
        Cancelled = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Active => "ACTIVE",
                Self::Succeeded => "SUCCEEDED",
                Self::Failed => "FAILED",
                Self::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
    /// Detailed log information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Appears only in the log created when the execution has started.
        #[prost(message, tag = "4")]
        Start(Start),
        /// Appears only in the log created when the execution has finished
        /// successfully.
        #[prost(message, tag = "5")]
        Success(Success),
        /// Appears only in the log created when the execution has failed.
        #[prost(message, tag = "6")]
        Failure(Failure),
    }
}
