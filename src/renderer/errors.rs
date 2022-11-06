use std::error::Error;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum CustomError {
    ParserFailure(String),
    HashingFailure(String),
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::ParserFailure(ref cause) => write!(f, "Parsing Error: {}", cause),
            CustomError::HashingFailure(ref cause) => {
                write!(f, "Unable to hash file content: {}", cause)
            }
        }
    }
}
