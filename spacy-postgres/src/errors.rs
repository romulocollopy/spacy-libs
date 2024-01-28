use std::fmt;

#[derive(Debug)]
pub struct SpacyDBError {
    pub message: String,
}

impl SpacyDBError {
    pub fn new(message: String) -> Self {
        SpacyDBError { message }
    }
}
impl std::error::Error for SpacyDBError {}

impl fmt::Display for SpacyDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpacyDBError: {}", &self.message)
    }
}
