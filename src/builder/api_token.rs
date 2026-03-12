// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Luke Harding <luke@lukeh990.io>

use super::PveBuilder;

impl PveBuilder {
    pub fn api_token<S: Into<String>>(mut self, token_id: S, secret: S) -> Self {
        let token_id = token_id.into();
        let secret = secret.into();

        self.api_token = Some((token_id, secret));

        self
    }
}
