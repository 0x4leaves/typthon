use super::lexer::Position;
use std::{error, fmt, result};

// This should return `Errors`, since we don't want to
// stop when the very first error is met. Reporting all the
// possible errors is better for users.
pub type Result<T> = result::Result<T, Errors>;

// Introduced a new type for Vec<Error> to handle error
// properly; Again, we need to go through all the source
// file even though an Error is generated.
#[derive(Debug)]
pub struct Errors(Vec<Error>);

#[derive(Debug)]
pub struct Error {
    // Keeping this field private let us to easily extend the
    // actual error variant (assuming the user of this library
    // will not care which error is returned).
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    LexError {
        file: String,
        msg: String,
        pos: Position,
    },
    ParseError {
        file: String,
        msg: String,
        pos: Position,
    },
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::LexError { file, msg, pos } => {
                // Refer to Serde's approach:
                write!(
                    f,
                    "Token Error: {} at {}:{}:{}",
                    msg, file, pos.line, pos.col
                )
            }
            ErrorKind::ParseError { file, msg, pos } => {
                write!(
                    f,
                    "Syntax Error: {} at {}:{}:{}",
                    msg, file, pos.line, pos.col
                )
            }
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let log: String = self.0.iter().map(|e| format!("{e}\n")).collect();
        write!(f, "{log}")
    }
}

impl error::Error for Error {}
impl error::Error for Errors {}
