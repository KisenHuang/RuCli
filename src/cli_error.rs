use std::io::Error as ioError;
use std::fmt::{Debug, Formatter, Display};

#[derive(Debug)]
pub struct CliError {
    ty: String,
    message: String,
}


pub enum ErrorType {
    CUSTOM,
    IO,
}

impl Into<String> for ErrorType {
    fn into(self) -> String {
        match self {
            ErrorType::CUSTOM => "custom",
            ErrorType::IO => "io",
        }.parse().unwrap()
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CliError({}: {})", self.ty, self.message)
    }
}

impl CliError {
    pub fn custom(msg: &str) -> Self {
        CliError {
            ty: ErrorType::CUSTOM.into(),
            message: msg.to_string(),
        }
    }

    pub fn io(err: ioError, msg: &str) -> Self {
        CliError {
            ty: ErrorType::IO.into(),
            message: format!("{} [{}]", err, msg),
        }
    }

}