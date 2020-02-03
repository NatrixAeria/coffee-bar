#[derive(Debug)]
pub enum XError {
    ConnError(xcb::base::ConnError),
}

impl std::fmt::Display for XError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            XError::ConnError(e) =>  write!(f, "display connection error [{}]", e),
        }
    }
}

impl std::error::Error for XError { }
