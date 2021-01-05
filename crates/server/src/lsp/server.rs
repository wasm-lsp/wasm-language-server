//! Definitions for the server instance.

use crate::core::session::Session;
use std::sync::Arc;

/// The WASM language server instance.
pub struct Server {
    /// The LSP client handle.
    pub(crate) client: lspower::Client,
    /// The current state of the server.
    pub(crate) session: Arc<Session>,
}

impl Server {
    /// Create a new server.
    pub fn new(client: lspower::Client) -> anyhow::Result<Self> {
        let session = Arc::new(Session::new(Some(client.clone()))?);
        Ok(Server { client, session })
    }
}

/// Compute the server capabilities.
pub fn capabilities() -> lsp::ServerCapabilities {
    let document_symbol_provider = Some(lsp::OneOf::Left(true));

    let hover_provider = Some(lsp::HoverProviderCapability::Simple(true));

    let semantic_tokens_provider = {
        let token_types = vec![
            lsp::SemanticTokenType::COMMENT,
            lsp::SemanticTokenType::FUNCTION,
            lsp::SemanticTokenType::KEYWORD,
            lsp::SemanticTokenType::NAMESPACE,
            lsp::SemanticTokenType::OPERATOR,
            lsp::SemanticTokenType::PARAMETER,
            lsp::SemanticTokenType::STRING,
            lsp::SemanticTokenType::TYPE,
            lsp::SemanticTokenType::TYPE_PARAMETER,
            lsp::SemanticTokenType::VARIABLE,
        ];
        let token_modifiers = Default::default();

        let options = lsp::SemanticTokensOptions {
            legend: lsp::SemanticTokensLegend {
                token_types,
                token_modifiers,
            },
            range: Some(true),
            full: Some(lsp::SemanticTokensFullOptions::Bool(true)),
            ..Default::default()
        };
        Some(lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options))
    };

    let text_document_sync = {
        let options = lsp::TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(lsp::TextDocumentSyncKind::Incremental),
            ..Default::default()
        };
        Some(lsp::TextDocumentSyncCapability::Options(options))
    };

    lsp::ServerCapabilities {
        document_symbol_provider,
        hover_provider,
        semantic_tokens_provider,
        text_document_sync,
        ..Default::default()
    }
}
