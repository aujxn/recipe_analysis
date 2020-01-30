use reqwest;

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
