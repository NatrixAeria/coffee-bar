#[derive(Debug, Clone)]
pub struct BarError(pub String);

impl std::fmt::Display for BarError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BarError {}

impl BarError {
    pub fn from_dis<D: std::fmt::Display>(d: D) -> Self {
        Self(format!("{}", d))
    }
}

impl Into<String> for BarError {
    fn into(self) -> String {
        self.0
    }
}
