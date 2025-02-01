// pub type Result<T> = core::result::Result<T, Error>;
// pub type Error = Box<dyn std::error::Error>;

use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {

    Env(String),

    ReadingHeaderFromRequest(String),

    Parsing(String),

    ThirdRequest(String),

    ServerStart(String),

    Paging(i32, String),

    Custom(i32, String),

    #[from]
    Io(std::io::Error)
}
impl std::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
