// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Luke Harding <luke@lukeh990.io>

use super::PveBuilder;

impl PveBuilder {
    pub fn base_url<S: Into<String>>(mut self, value: S) -> Self {
        let value = value.into();
        self.base_url = Some(value);

        self
    }
}
