#[derive(Debug)]
pub enum XError {
    ConnError(xcb::base::ConnError),
    ScreenError(String),
    XcbError(String),
}

impl std::fmt::Display for XError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            XError::ConnError(e) =>  write!(f, "display connection error [{}]", e),
            XError::ScreenError(e) =>  write!(f, "{}", e),
            XError::XcbError(e) =>  write!(f, "xcb {}", e),
        }
    }
}

impl std::error::Error for XError { }

impl From<xcb::base::GenericError> for XError {
    fn from(e: xcb::base::GenericError) -> Self {
        XError::XcbError(format!("generic error (type: {}, code: {})",
            e.response_type(),
            e.error_code()))
    }
}
