use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("Failed to build reqwest client")]
    ReqwestBuild(reqwest::Error),
    #[error("No API Token")]
    NoToken,
    #[error("Invalid Header Value")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("No Base URL")]
    NoBase,
    #[error("Invalid URL")]
    InvalidUrl(#[from] url::ParseError),
}
