// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Luke Harding <luke@lukeh990.io>

use crate::{Pve, PveError};
use reqwest::StatusCode;
use serde_json::Value;
use std::collections::HashMap;

/// Build up a request. Usually this just allows you to add parameters before
/// sending.
#[derive(Debug)]
pub struct RequestBuilder {
    path: String,
    method: RequestMethod,
    parameters: Option<HashMap<String, String>>,
    pve: Pve,
}

/// Describes the HTTP method in use.
#[derive(Debug)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl RequestBuilder {
    /// Creates a new [`RequestBuilder`] pulling client and base URL from
    /// [`Pve`]. Path specifies the sub path for the endpoint.
    pub fn new(pve: &Pve, path: String, method: RequestMethod) -> Self {
        RequestBuilder {
            path,
            method,
            parameters: None,
            pve: pve.clone(),
        }
    }

    /// Since this will always be encoded in www-form-urlencoded it can be
    /// represented in a [`HashMap<String, String>`] just fine. This will
    /// init and add to a HashMap which is encoded when [`Self::send`] is
    /// called.
    pub fn add_parameter<S: Into<String>>(mut self, name: S, value: S) -> Self {
        let map = self.parameters.get_or_insert_with(HashMap::new);

        map.insert(name.into(), value.into());

        self
    }

    /// Create and send the HTTP request. Response will come back as a
    /// [`serde_json::Value`] type.
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
