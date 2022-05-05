use std::fmt;
pub enum CustomError {
    ParserFailure(String),
    HashingFailure(String),
}

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
