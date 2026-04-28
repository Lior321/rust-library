use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EsmError {
    Io(std::io::Error),
    InvalidArgument(String),
}

impl fmt::Display for EsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EsmError::Io(err) => write!(f, "IO error: {}", err),
            EsmError::InvalidArgument(msg) => write!(f, "Invalid argument received: {}", msg),
        }
    }
}

impl Error for EsmError {
    // source() allows you to "reach through" your error to the underlying cause
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            EsmError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for EsmError {
    fn from(err: std::io::Error) -> EsmError {
        EsmError::Io(err)
    }
}
