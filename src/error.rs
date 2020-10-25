use std::{fmt, io};

#[derive(Debug)]
pub enum ImagextractorError {
    InvalidArgs(String),
    IOError(io::Error),
    Other,
}

impl fmt::Display for ImagextractorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ImagextractorError::Other => write!(f, "Other Error"),
            ImagextractorError::InvalidArgs(ref err) => write!(f, "Invalide arguments: {}", err),
            ImagextractorError::IOError(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ImagextractorError {
    fn description(&self) -> &str {
        "ImagextractorError"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}
