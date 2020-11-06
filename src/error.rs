use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest error:{0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("parse error:{0}")]
    UrlParseError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
