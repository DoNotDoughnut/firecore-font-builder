#[derive(Debug)]
pub enum FontError {

    IOError(std::io::Error),
    ParseError(String, ron::Error),
    SerializeError(bincode::Error),

}

impl std::error::Error for FontError {}

impl From<std::io::Error> for FontError {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<bincode::Error> for FontError {
    fn from(err: bincode::Error) -> Self {
        Self::SerializeError(err)
    }
}

impl core::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}