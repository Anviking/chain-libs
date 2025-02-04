use std::{error, fmt};

/// Common error codes for network protocol requests.
///
/// These codes mimic the status codes used in gRPC and map one to one to
/// those in the gRPC protocol implementation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Code {
    Canceled,
    Unknown,
    InvalidArgument,
    NotFound,
    FailedPrecondition,
    Aborted,
    Unimplemented,
    Internal,
    Unavailable,
}

/// Represents errors that can be returned by the node protocol implementation.
#[derive(Debug)]
pub struct Error {
    code: Code,
    source: Box<dyn error::Error + Send + Sync>,
}

impl Error {
    pub fn new<E>(code: Code, source: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Error {
            code,
            source: source.into(),
        }
    }

    pub fn code(&self) -> Code {
        self.code
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self.code {
            Code::Canceled => "processing canceled",
            Code::Unknown => "unknown error",
            Code::InvalidArgument => "invalid request data",
            Code::NotFound => "not found",
            Code::FailedPrecondition => "system state does not permit the operation",
            Code::Aborted => "the operation was aborted",
            Code::Unimplemented => "not implemented",
            Code::Internal => "internal processing error",
            Code::Unavailable => "the service is unavailable",
        };
        f.write_str(msg)
    }
}
