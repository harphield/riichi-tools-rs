use std::error::Error;
use std::fmt::Formatter;
use std::fmt::Result;
use wasm_bindgen::__rt::core::fmt::Display;

/// An error we propagate in the module
#[derive(Debug)]
pub struct RiichiError {
    /// A numeric code
    pub code: u16,
    /// String message
    pub message: String,
}

impl RiichiError {
    /// Create a new RiichiError
    pub fn new(code: u16, message: &str) -> RiichiError {
        RiichiError {
            code,
            message: message.to_string(),
        }
    }
}

impl Display for RiichiError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} : {}", self.code, self.message)
    }
}

impl Error for RiichiError {
    fn description(&self) -> &str {
        &self.message
    }
}
