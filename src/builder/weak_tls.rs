// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Luke Harding <luke@lukeh990.io>

use super::PveBuilder;

impl PveBuilder {
    pub fn weak_tls(mut self, value: bool) -> Self {
        self.weak_tls = Some(value);

        self
    }
}
