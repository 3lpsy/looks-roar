use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub msg: String,
    pub code: u8,
}

impl AppError {
    pub fn new(msg: String, code: u8) -> Self {
        Self { msg, code }
    }
    pub fn boxed(msg: String, code: u8) -> Box<Self> {
        Box::new(Self { msg, code })
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.msg)
    }
}

impl Error for AppError {}
