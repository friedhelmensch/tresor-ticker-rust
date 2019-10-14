use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was a problem ey")
    }
}

impl Error {
    pub fn new(error_message: String) -> Error {
        Error {
            message: error_message,
        }
    }
}
