use crate::{Pve, PveError};
use reqwest::StatusCode;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RequestBuilder {
    path: String,
    method: RequestMethod,
    parameters: Option<HashMap<String, String>>,
    pve: Pve,
}

#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl RequestBuilder {
    pub fn new(pve: &Pve, path: String, method: RequestMethod) -> Self {
        RequestBuilder {
            path,
            method,
            parameters: None,
            pve: pve.clone(),
        }
    }

    pub fn add_parameter<S: Into<String>>(mut self, name: S, value: S) -> Self {
        let map = self.parameters.get_or_insert_with(HashMap::new);

        map.insert(name.into(), value.into());

        self
    }

    pub async fn send(self) -> Result<Value, PveError> {
        let url = self.pve.base_url.join(&self.path)?;

        let builder = if let Some(parameters) = self.parameters {
            match self.method {
                RequestMethod::Get => self.pve.client.get(url).query(&parameters),
                RequestMethod::Post => self.pve.client.post(url).form(&parameters),
                RequestMethod::Put => self.pve.client.put(url).form(&parameters),
                RequestMethod::Delete => self.pve.client.delete(url).query(&parameters),
            }
        } else {
            match self.method {
                RequestMethod::Get => self.pve.client.get(url),
                RequestMethod::Post => self.pve.client.post(url),
                RequestMethod::Put => self.pve.client.put(url),
                RequestMethod::Delete => self.pve.client.delete(url),
            }
        };

        let response = builder.send().await?;
        let status = response.status();

        if status == StatusCode::OK {
            Ok(response.json().await?)
        } else {
            Err(Pve::process_err_resp(status, response.text().await?))
        }
    }
}
