use reqwest;
use serde_json;
use url::ParseError;

#[derive(Debug)]
pub enum Error {
    ScrapeError,
}

impl From<reqwest::Error> for Error {
    fn from(_other: reqwest::Error) -> Self {
        Error::ScrapeError
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(_other: std::num::ParseFloatError) -> Self {
        Error::ScrapeError
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_other: std::num::ParseIntError) -> Self {
        Error::ScrapeError
    }
}

impl From<url::ParseError> for Error {
    fn from(_other: url::ParseError) -> Self {
        Error::ScrapeError
    }
}

impl From<serde_json::Error> for Error {
    fn from(_other: serde_json::Error) -> Self {
        Error::ScrapeError
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_other: std::convert::Infallible) -> Self {
        Error::ScrapeError
    }
}
