use core::fmt::Display;

use uacpi_sys::{
    uacpi_status_to_string, UACPI_STATUS_ALREADY_EXISTS, UACPI_STATUS_AML_BAD_ENCODING,
    UACPI_STATUS_AML_CALL_STACK_DEPTH_LIMIT, UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE,
    UACPI_STATUS_AML_INVALID_NAMESTRING, UACPI_STATUS_AML_INVALID_OPCODE,
    UACPI_STATUS_AML_INVALID_RESOURCE, UACPI_STATUS_AML_LOOP_TIMEOUT,
    UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS, UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX,
    UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH, UACPI_STATUS_AML_UNDEFINED_REFERENCE,
    UACPI_STATUS_BAD_CHECKSUM, UACPI_STATUS_COMPILED_OUT, UACPI_STATUS_DENIED,
    UACPI_STATUS_HARDWARE_TIMEOUT, UACPI_STATUS_INIT_LEVEL_MISMATCH, UACPI_STATUS_INTERNAL_ERROR,
    UACPI_STATUS_INVALID_SIGNATURE, UACPI_STATUS_INVALID_TABLE_LENGTH, UACPI_STATUS_MAPPING_FAILED,
    UACPI_STATUS_NAMESPACE_NODE_DANGLING, UACPI_STATUS_NOT_FOUND, UACPI_STATUS_NO_HANDLER,
    UACPI_STATUS_NO_RESOURCE_END_TAG, UACPI_STATUS_OK, UACPI_STATUS_OUT_OF_MEMORY,
    UACPI_STATUS_OVERRIDDEN, UACPI_STATUS_TIMEOUT, UACPI_STATUS_TYPE_MISMATCH,
    UACPI_STATUS_UNIMPLEMENTED,
};

#[must_use]
#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Ok = UACPI_STATUS_OK,
    MappingFailed = UACPI_STATUS_MAPPING_FAILED,
    OutOfMemory = UACPI_STATUS_OUT_OF_MEMORY,
    BadChecksum = UACPI_STATUS_BAD_CHECKSUM,
    InvalidSignature = UACPI_STATUS_INVALID_SIGNATURE,
    InvalidTableLength = UACPI_STATUS_INVALID_TABLE_LENGTH,
    NotFound = UACPI_STATUS_NOT_FOUND,
    Unimplemented = UACPI_STATUS_UNIMPLEMENTED,
    AlreadyExists = UACPI_STATUS_ALREADY_EXISTS,
    InternalError = UACPI_STATUS_INTERNAL_ERROR,
    TypeMismatch = UACPI_STATUS_TYPE_MISMATCH,
    InitLevelMismatch = UACPI_STATUS_INIT_LEVEL_MISMATCH,
    NamespaceNodeDangling = UACPI_STATUS_NAMESPACE_NODE_DANGLING,
    NoHandler = UACPI_STATUS_NO_HANDLER,
    NoResourceEndTag = UACPI_STATUS_NO_RESOURCE_END_TAG,
    CompiledOut = UACPI_STATUS_COMPILED_OUT,
    HardwareTimeout = UACPI_STATUS_HARDWARE_TIMEOUT,
    Timeout = UACPI_STATUS_TIMEOUT,
    Overriden = UACPI_STATUS_OVERRIDDEN,
    Denied = UACPI_STATUS_DENIED,

    AmlUndefinedReference = UACPI_STATUS_AML_UNDEFINED_REFERENCE,
    AmlInvalidNameString = UACPI_STATUS_AML_INVALID_NAMESTRING,
    AmlObjectAlreadyExists = UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS,
    AmlInvalidOpcode = UACPI_STATUS_AML_INVALID_OPCODE,
    AmlIncompatibleObjectType = UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE,
    AmlBadEncoding = UACPI_STATUS_AML_BAD_ENCODING,
    AmlOutOfBoundsIndex = UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX,
    AmlSyncLevelTooHigh = UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH,
    AmlInvalidResource = UACPI_STATUS_AML_INVALID_RESOURCE,
    AmlLoopTimeout = UACPI_STATUS_AML_LOOP_TIMEOUT,
    AmlCallStackDepthLimit = UACPI_STATUS_AML_CALL_STACK_DEPTH_LIMIT,
}

impl From<uacpi_sys::uacpi_status> for Status {
    fn from(status: uacpi_sys::uacpi_status) -> Self {
        match status {
            UACPI_STATUS_OK => Status::Ok,
            UACPI_STATUS_MAPPING_FAILED => Status::MappingFailed,
            UACPI_STATUS_OUT_OF_MEMORY => Status::OutOfMemory,
            UACPI_STATUS_BAD_CHECKSUM => Status::BadChecksum,
            UACPI_STATUS_INVALID_SIGNATURE => Status::InvalidSignature,
            UACPI_STATUS_INVALID_TABLE_LENGTH => Status::InvalidTableLength,
            UACPI_STATUS_NOT_FOUND => Status::NotFound,
            UACPI_STATUS_UNIMPLEMENTED => Status::Unimplemented,
            UACPI_STATUS_ALREADY_EXISTS => Status::AlreadyExists,
            UACPI_STATUS_INTERNAL_ERROR => Status::InternalError,
            UACPI_STATUS_TYPE_MISMATCH => Status::TypeMismatch,
            UACPI_STATUS_INIT_LEVEL_MISMATCH => Status::InitLevelMismatch,
            UACPI_STATUS_NAMESPACE_NODE_DANGLING => Status::NamespaceNodeDangling,
            UACPI_STATUS_NO_HANDLER => Status::NoHandler,
            UACPI_STATUS_NO_RESOURCE_END_TAG => Status::NoResourceEndTag,
            UACPI_STATUS_COMPILED_OUT => Status::CompiledOut,
            UACPI_STATUS_HARDWARE_TIMEOUT => Status::HardwareTimeout,
            UACPI_STATUS_TIMEOUT => Status::Timeout,
            UACPI_STATUS_OVERRIDDEN => Status::Overriden,
            UACPI_STATUS_DENIED => Status::Denied,

            UACPI_STATUS_AML_UNDEFINED_REFERENCE => Status::AmlUndefinedReference,
            UACPI_STATUS_AML_INVALID_NAMESTRING => Status::AmlInvalidNameString,
            UACPI_STATUS_AML_OBJECT_ALREADY_EXISTS => Status::AmlObjectAlreadyExists,
            UACPI_STATUS_AML_INVALID_OPCODE => Status::AmlInvalidOpcode,
            UACPI_STATUS_AML_INCOMPATIBLE_OBJECT_TYPE => Status::AmlIncompatibleObjectType,
            UACPI_STATUS_AML_BAD_ENCODING => Status::AmlBadEncoding,
            UACPI_STATUS_AML_OUT_OF_BOUNDS_INDEX => Status::AmlOutOfBoundsIndex,
            UACPI_STATUS_AML_SYNC_LEVEL_TOO_HIGH => Status::AmlSyncLevelTooHigh,
            UACPI_STATUS_AML_INVALID_RESOURCE => Status::AmlInvalidResource,
            UACPI_STATUS_AML_LOOP_TIMEOUT => Status::AmlLoopTimeout,
            UACPI_STATUS_AML_CALL_STACK_DEPTH_LIMIT => Status::AmlCallStackDepthLimit,
            _ => panic!("Uknown uACPI status value: {status:#x}"),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string = unsafe { uacpi_status_to_string(*self as u32) };
        let string = unsafe { core::ffi::CStr::from_ptr(string) };

        writeln!(f, "{string:?}")
    }
}
