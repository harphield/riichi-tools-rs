use std::error::Error;
use wasm_bindgen::__rt::core::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct RiichiError {
    pub code: u16,
    pub message: String,
}

impl RiichiError {
    pub fn new(code: u16, message: &str) -> RiichiError {
        RiichiError {
            code,
            message: message.to_string()
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