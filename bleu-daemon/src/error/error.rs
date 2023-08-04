use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum DaemonError {
	Connection(String),
}

impl Display for DaemonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DaemonError::Connection(s) => write!(f, "{}", s),
        }
    }
}
