mod api_token;
mod base_url;
mod error;
mod user_agent;
mod weak_tls;

use crate::Pve;
use error::BuildError;
use reqwest::{Client, Url, header};

#[derive(Default, Debug)]
pub struct PveBuilder {
    user_agent: Option<String>,
    api_token: Option<(String, String)>,
    base_url: Option<String>,
    weak_tls: Option<bool>,
}

impl PveBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> Result<Pve, BuildError> {
        let user_agent_def = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let user_agent = self.user_agent.unwrap_or(user_agent_def);

        let mut header_map = header::HeaderMap::new();

        let api_token = self.api_token.ok_or(BuildError::NoToken)?;
        let mut api_token_header =
            header::HeaderValue::from_str(&format!("PVEAPIToken={}={}", api_token.0, api_token.1))?;
        api_token_header.set_sensitive(true);
        header_map.insert(header::AUTHORIZATION, api_token_header);

        let weak_tls = self.weak_tls.unwrap_or(false);

        let client_builder = Client::builder()
            .user_agent(user_agent)
            .default_headers(header_map)
            .tls_danger_accept_invalid_certs(weak_tls);

        let client = client_builder.build().map_err(BuildError::ReqwestBuild)?;

        let base_url_str = self.base_url.ok_or(BuildError::NoBase)?;
        let mut base_url = Url::parse(&base_url_str)?;

        base_url.set_path("/api2/json/");

        Ok(Pve::new(client, base_url))
    }
}
