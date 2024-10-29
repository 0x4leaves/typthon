pub mod error;
pub mod lexer;

pub use error::{Error, Result};

pub fn compile(src: &str) -> Result<String> {
    todo!()
}
