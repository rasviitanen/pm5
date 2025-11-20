use std::io::Cursor;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid bytes")]
    Io(#[from] std::io::Error),
    #[error("invalid variant")]
    Variant,
    #[error("unexpected number of bytes")]
    UnexpectedNumberOfBytes,
}

pub trait Parse: Sized {
    fn parse(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, ParseError>;
}
