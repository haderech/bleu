use hex::FromHexError;
use lettre::transport::smtp;
use std::{
	env::VarError,
	fmt::{Display, Formatter},
	num::ParseIntError,
	str::ParseBoolError,
	string::FromUtf8Error,
};

#[derive(Debug, Clone)]
pub enum ExpectedError {
	TypeError(String),
	NoneError(String),
	ProcessError(String),
	InvalidError(String),
	RequestError(String),
	ParsingError(String),
	ChannelError(String),
	UnknownBlockError(String),
	PostgresError(String),
	IoError(String),
	JsonRpcError(String),
	ConnectionError(String),
}

impl From<smtp::Error> for ExpectedError {
	fn from(err: smtp::Error) -> Self {
		ExpectedError::ProcessError(err.to_string())
	}
}

impl From<ParseBoolError> for ExpectedError {
	fn from(err: ParseBoolError) -> Self {
		ExpectedError::TypeError(err.to_string())
	}
}

impl From<VarError> for ExpectedError {
	fn from(err: VarError) -> Self {
		match err {
			VarError::NotPresent => ExpectedError::NoneError(err.to_string()),
			VarError::NotUnicode(_) => ExpectedError::TypeError(err.to_string()),
		}
	}
}

impl From<reqwest::Error> for ExpectedError {
	fn from(err: reqwest::Error) -> Self {
		ExpectedError::RequestError(err.to_string())
	}
}

impl From<serde_json::Error> for ExpectedError {
	fn from(err: serde_json::Error) -> Self {
		ExpectedError::ParsingError(err.to_string())
	}
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for ExpectedError {
	fn from(err: tokio::sync::broadcast::error::SendError<T>) -> Self {
		ExpectedError::ChannelError(err.to_string())
	}
}

impl From<ParseIntError> for ExpectedError {
	fn from(err: ParseIntError) -> Self {
		ExpectedError::ParsingError(err.to_string())
	}
}

impl From<r2d2_postgres::postgres::Error> for ExpectedError {
	fn from(err: r2d2_postgres::postgres::Error) -> Self {
		ExpectedError::PostgresError(err.to_string())
	}
}

impl From<std::io::Error> for ExpectedError {
	fn from(err: std::io::Error) -> Self {
		ExpectedError::IoError(err.to_string())
	}
}

impl From<FromUtf8Error> for ExpectedError {
	fn from(err: FromUtf8Error) -> Self {
		ExpectedError::ParsingError(err.to_string())
	}
}

impl From<FromHexError> for ExpectedError {
	fn from(err: FromHexError) -> Self {
		ExpectedError::ParsingError(err.to_string())
	}
}

impl From<jsonrpc_core::Error> for ExpectedError {
	fn from(err: jsonrpc_core::Error) -> Self {
		ExpectedError::JsonRpcError(err.to_string())
	}
}

impl Display for ExpectedError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ExpectedError::TypeError(err) => write!(f, "{}", err),
			ExpectedError::NoneError(err) => write!(f, "{}", err),
			ExpectedError::ProcessError(err) => write!(f, "{}", err),
			ExpectedError::InvalidError(err) => write!(f, "{}", err),
			ExpectedError::RequestError(err) => write!(f, "{}", err),
			ExpectedError::ParsingError(err) => write!(f, "{}", err),
			ExpectedError::ChannelError(err) => write!(f, "{}", err),
			ExpectedError::UnknownBlockError(err) => write!(f, "{}", err),
			ExpectedError::PostgresError(err) => write!(f, "{}", err),
			ExpectedError::IoError(err) => write!(f, "{}", err),
			ExpectedError::JsonRpcError(err) => write!(f, "{}", err),
			ExpectedError::ConnectionError(err) => write!(f, "{}", err),
		}
	}
}
