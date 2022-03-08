use std::fmt;

#[derive(Debug, Clone)]
pub struct RgmError {
    pub message: String
}
pub type Result<T> = std::result::Result<T, RgmError>;

impl fmt::Display for RgmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
