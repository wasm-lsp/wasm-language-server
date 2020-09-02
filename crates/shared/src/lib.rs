//! The WebAssembly language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

// Core definitions for the WebAssembly language server.
pub mod core;

// LSP-related definitions for the WebAssembly language server.
pub mod lsp;