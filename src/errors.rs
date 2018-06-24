use std;
use ring;
use base64;

#[derive(Debug)]
pub enum ExampleError {
    LibraryError(&'static str, Box<std::error::Error>),
    IoError(&'static str, Box<std::error::Error>),
    AssertionMismatch(),
}

impl std::fmt::Display for ExampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ExampleError::LibraryError(lib_name, ref err) => write!(f, "Library {} error: {}", lib_name, err),
            ExampleError::IoError(io_name, ref err) => write!(f, "IO {} error: {}", io_name, err),
            ExampleError::AssertionMismatch() => write!(f, "Mismatching original and decrypted data"),
        }
    }
}

impl std::error::Error for ExampleError {
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ExampleError::LibraryError(_, ref err) => Some(err.as_ref()),
            ExampleError::IoError(_, ref err) => Some(err.as_ref()),
            ExampleError::AssertionMismatch() => None,
        }
    }
}

impl From<ring::error::Unspecified> for ExampleError {
    fn from(err: ring::error::Unspecified) -> ExampleError {
        ExampleError::LibraryError("ring", Box::new(err))
    }
}

impl From<base64::DecodeError> for ExampleError {
    fn from(err: base64::DecodeError) -> ExampleError {
        ExampleError::IoError("base64", Box::new(err))
    }
}

impl From<std::io::Error> for ExampleError {
    fn from(err: std::io::Error) -> ExampleError {
        ExampleError::IoError("std::io", Box::new(err))
    }
}

impl From<std::string::FromUtf8Error> for ExampleError {
    fn from(err: std::string::FromUtf8Error) -> ExampleError {
        ExampleError::IoError("UTF8", Box::new(err))
    }
}