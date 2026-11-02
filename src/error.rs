use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("corrupt record at line {line_no}: {line:?}")]
    Corrupt { line_no: usize, line: String },
}

pub type Result<T> = std::result::Result<T, Error>;
