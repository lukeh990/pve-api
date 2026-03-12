//! Simple API
//! This API is not type checked and doesn't validate anything you provide.
//!
//! To use:
//!
//! - Add [`SimpleApi`] to use statement
//! - The [`SimpleApi`] functions will be accessible from any [`crate::Pve`]

// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Luke Harding <luke@lukeh990.io>

mod pve;
mod request;

pub use request::RequestBuilder;

/// You almost certainly don't want to implement this. Do so at your own risk.
/// Check [`crate::Pve`] for docs.
pub trait SimpleApi {
    fn simple_get<S: Into<String>>(&self, path: S) -> RequestBuilder;

    fn simple_post<S: Into<String>>(&self, path: S) -> RequestBuilder;

    fn simple_put<S: Into<String>>(&self, path: S) -> RequestBuilder;

    fn simple_delete<S: Into<String>>(&self, path: S) -> RequestBuilder;
}
