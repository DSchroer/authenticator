use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct Error {
    message: String
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.message, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.message, f)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn from(message: &str) -> Self {
        Error{ message: String::from(message) }
    }
}