use std::fmt;
use std::error::Error as StdError;  // Import the Error trait
use reqwest::Error as ReqwestError;

pub struct UnipusError {
    pub message: String,  // Error message
}

impl fmt::Display for UnipusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnipusError: {}", self.message)
    }
}

impl fmt::Debug for UnipusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnipusError: {}", self.message)
    }
}

// Implementing the `Error` trait
impl StdError for UnipusError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None // No underlying cause/error
    }
}

impl From<ReqwestError> for UnipusError {
    fn from(err: ReqwestError) -> Self {
        UnipusError {
            message: err.to_string(),
        }
    }
}

impl UnipusError {
    pub fn new(message: &str) -> Self {
        UnipusError {
            message: message.to_string(),
        }
    }
}
