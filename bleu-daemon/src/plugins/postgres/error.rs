use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum PostgresError {
	UnsupportedType(String),
	Connection(String),
	ExecutedFailed(String),
	InvalidType(String),
}

impl Display for PostgresError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresError::UnsupportedType(s) => write!(f, "{}", s),
            PostgresError::Connection(s) => write!(f, "{}", s),
            PostgresError::ExecutedFailed(s) => write!(f, "{}", s),
            PostgresError::InvalidType(s) => write!(f, "{}", s),
        }
    }
}
