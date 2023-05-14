use io::Error as IoError;
use napi::{Error as NapiError, Status};
use opencl3::error_codes::ClError;
use opencl3::types::cl_int;
use std::io;

// custom cl error codes
pub const INVALID_GLOBAL_ARRAY_ID: cl_int = -200;
pub const INVALID_BUFFER_LEN: cl_int = -201;
pub const GLOBAL_ARRAY_ID_ASSIGNED: cl_int = -202;
pub const NO_GLOBAL_VECTORS_TO_ASSIGN: cl_int = -203;
pub const INVALID_KERNEL_BLOCK_NAME: cl_int = -204;

#[derive(Debug)]
pub enum OpenclError {
    OpenCl(cl_int),       // original opencl error code
    CustomOpenCl(cl_int), // custom cl error code
    Other,
    // Napi(NapiError)
    // Io(String)w
}

pub type OpenClResult<T> = Result<T, OpenclError>;

impl std::error::Error for OpenclError {}

impl std::fmt::Display for OpenclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenclError::OpenCl(v) => write!(f, "opencl error code: {v}"),
            OpenclError::CustomOpenCl(v) => write!(f, "custom opencl error code: {v}"),
            OpenclError::Other => write!(f, "other error"),
            // OpenclError::Napi(v) => write!(f, "{}", v),
        }
    }
}

impl From<IoError> for OpenclError {
    fn from(_: IoError) -> Self {
        // match value.kind() {
        //     ErrorKind::NotFound => {}
        //     ErrorKind::PermissionDenied => {}
        //     ErrorKind::ConnectionRefused => {}
        //     ErrorKind::ConnectionReset => {}
        //     ErrorKind::HostUnreachable => {}
        //     ErrorKind::NetworkUnreachable => {}
        //     ErrorKind::ConnectionAborted => {}
        //     ErrorKind::NotConnected => {}
        //     ErrorKind::AddrInUse => {}
        //     ErrorKind::AddrNotAvailable => {}
        //     ErrorKind::NetworkDown => {}
        //     ErrorKind::BrokenPipe => {}
        //     ErrorKind::AlreadyExists => {}
        //     ErrorKind::WouldBlock => {}
        //     ErrorKind::NotADirectory => {}
        //     ErrorKind::IsADirectory => {}
        //     ErrorKind::DirectoryNotEmpty => {}
        //     ErrorKind::ReadOnlyFilesystem => {}
        //     ErrorKind::FilesystemLoop => {}
        //     ErrorKind::StaleNetworkFileHandle => {}
        //     ErrorKind::InvalidInput => {}
        //     ErrorKind::InvalidData => {}
        //     ErrorKind::TimedOut => {}
        //     ErrorKind::WriteZero => {}
        //     ErrorKind::StorageFull => {}
        //     ErrorKind::NotSeekable => {}
        //     ErrorKind::FilesystemQuotaExceeded => {}
        //     ErrorKind::FileTooLarge => {}
        //     ErrorKind::ResourceBusy => {}
        //     ErrorKind::ExecutableFileBusy => {}
        //     ErrorKind::Deadlock => {}
        //     ErrorKind::CrossesDevices => {}
        //     ErrorKind::TooManyLinks => {}
        //     ErrorKind::InvalidFilename => {}
        //     ErrorKind::ArgumentListTooLong => {}
        //     ErrorKind::Interrupted => {}
        //     ErrorKind::Unsupported => {}
        //     ErrorKind::UnexpectedEof => {}
        //     ErrorKind::OutOfMemory => {}
        //     ErrorKind::Other => {}
        //     ErrorKind::Uncategorized => {}
        // }
        OpenclError::Other
    }
}

impl From<ClError> for OpenclError {
    fn from(e: ClError) -> Self {
        Self::OpenCl(e.0)
    }
}

// impl From<JsError> for OpenclError {
//     fn from(_: JsError) -> Self {
//         OpenclError::Other
//     }
// }

impl From<NapiError> for OpenclError {
    fn from(_: NapiError) -> Self {
        // match e.status {
        //     Status::Ok => {}
        //     Status::InvalidArg => {}
        //     Status::ObjectExpected => {}
        //     Status::StringExpected => {}
        //     Status::NameExpected => {}
        //     Status::FunctionExpected => {}
        //     Status::NumberExpected => {}
        //     Status::BooleanExpected => {}
        //     Status::ArrayExpected => {}
        //     Status::GenericFailure => {}
        //     Status::PendingException => {}
        //     Status::Cancelled => {}
        //     Status::EscapeCalledTwice => {}
        //     Status::HandleScopeMismatch => {}
        //     Status::CallbackScopeMismatch => {}
        //     Status::QueueFull => {}
        //     Status::Closing => {}
        //     Status::BigintExpected => {}
        //     Status::DateExpected => {}
        //     Status::ArrayBufferExpected => {}
        //     Status::DetachableArraybufferExpected => {}
        //     Status::WouldDeadlock => {}
        //     Status::NoExternalBuffersAllowed => {}
        //     Status::Unknown => {}
        // }
        OpenclError::Other
    }
}

impl From<OpenclError> for NapiError {
    fn from(e: OpenclError) -> Self {
        match e {
            OpenclError::OpenCl(v) => Self::new(Status::Unknown, format!("opencl error code: {v}")),
            OpenclError::CustomOpenCl(v) => {
                Self::new(Status::Unknown, format!("custom opencl error code: {v}"))
            }
            OpenclError::Other => Self::new(Status::Unknown, format!("unknown opencl error")),
        }
    }
}

impl From<OpenclError> for IoError {
    fn from(e: OpenclError) -> Self {
        match e {
            OpenclError::OpenCl(v) => {
                Self::new(io::ErrorKind::Other, format!("opencl error code: {v}"))
            }
            OpenclError::CustomOpenCl(v) => Self::new(
                io::ErrorKind::Other,
                format!("custom opencl error code: {v}"),
            ),
            OpenclError::Other => Self::new(io::ErrorKind::Other, format!("unknown opencl error")),
        }
    }
}
