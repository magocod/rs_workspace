use napi::{Error as NapiError, Status};
use opencl3::error_codes::ClError;
use opencl3::types::cl_int;

// custom cl error codes
pub const INVALID_GLOBAL_ARRAY_ID: cl_int = -200;
pub const INVALID_BUFFER_LEN: cl_int = -201;
pub const GLOBAL_ARRAY_ID_ASSIGNED: cl_int = -202;
pub const NO_GLOBAL_VECTORS_TO_ASSIGN: cl_int = -203;

#[derive(Debug)]
pub enum OpenclError {
    OpenCl(cl_int),       // original opencl error code
    CustomOpenCl(cl_int), // custom cl error code
    Other,
    // Napi(NapiError)
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
