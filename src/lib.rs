#![doc = include_str!("../README.md")]

pub mod builder;
pub mod simple;

pub use builder::PveBuilder;
use reqwest::{Client, StatusCode, Url};
pub use simple::SimpleApi;
use thiserror::Error;

/// This is the main struct for this project. It manages all the state
/// required. Usually built with [`PveBuilder`]
#[derive(Debug, Clone)]
pub struct Pve {
    client: Client,
    base_url: Url,
}

impl Pve {
    /// Create a new builder for [`Pve`]
    ///
    /// For Example:
    /// ```
    /// var pve: Pve = Pve::builder()
    ///     .base_url("<PVE_URL>")
    ///     .api_token("<TOKEN_ID>", "<SECRET>")
    ///     .build()?;
    /// ```
    pub fn builder() -> PveBuilder {
        PveBuilder::new()
    }

    fn new(client: Client, base_url: Url) -> Self {
        Pve { client, base_url }
    }

    fn process_err_resp(status: StatusCode, body: String) -> PveError {
        if status == StatusCode::UNAUTHORIZED {
            PveError::Unauthorized(body)
        } else if status == StatusCode::NOT_FOUND {
            PveError::NotFound(body)
        } else if status.is_client_error() {
            PveError::Client(body)
        } else if status.is_server_error() {
            PveError::Server(body)
        } else {
            PveError::Unknown(body)
        }
    }

    /// Run connection tests against the configured remote server.
    ///
    /// For Example:
    /// ```
    /// var pve: Pve = ...; // See Pve::builder()
    /// var test: Result<(), PveError> = pve.test()
    /// ```
    pub async fn test(&self) -> Result<(), PveError> {
        let path = self.base_url.join("version")?;
        let result = self.client.get(path).send().await?;
        let status = result.status();

        if status == StatusCode::OK {
            Ok(())
        } else {
            Err(Self::process_err_resp(status, result.text().await?))
        }
    }
}

/// Used as a very broad error type encompassing serialization, request
/// construction, parsing URLs, and response status codes. If you want to
/// actually understand why something failed, you'd probably want to use a
/// match statement.
#[derive(Debug, Error)]
pub enum PveError {
    #[error("Failed to parse URL")]
    UrlParse(#[from] url::ParseError),
    #[error("Error constructing / sending request")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Client Error (4XX) : {0}")]
    Client(String),
    #[error("Server Error (5XX) : {0}")]
    Server(String),
    #[error("Unknown Error: {0}")]
    Unknown(String),
    #[error("Serialization Error")]
    Serialize(serde_json::Error),
    #[error("Deserialization Error")]
    Deserialize(serde_json::Error),
}
