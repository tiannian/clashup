use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    /* #[error("the data for key `{0}` is not available")] */
    /* Redaction(String), */
    /* #[error("invalid header (expected {expected:?}, found {found:?})")] */
    /* InvalidHeader { */
    /*     expected: String, */
    /*     found: String, */
    /* }, */
    /* #[error("unknown data store error")] */
    /* Unknown, */
    #[error("exit code error.")]
    ExitCodeError,
    #[error("Please execute `clashup init`")]
    NoClashBiniar,
}

pub type Result<T> = std::result::Result<T, Error>;
