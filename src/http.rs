// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! Streamable HTTP transport (MCP 2025-03-26 and later).
//!
//! Exposes the same tools as the stdio transport, so hosts that connect over a
//! URL rather than a child process — Claude Desktop's custom connectors, for
//! instance — can use the server. The MCP endpoint lives at [`MCP_PATH`];
//! `GET /health` is a plain liveness probe for supervised deployments.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::{Context, Result};
use axum::{Router, routing::get};
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};

use crate::server::LatServer;

/// Path of the MCP endpoint. Clients are configured with `http://host:port/mcp`.
pub const MCP_PATH: &str = "/mcp";

/// Builds the HTTP router. Each session gets its own [`LatServer`]; the
/// database path is the only shared state and connections stay per call.
///
/// `extra_allowed_hosts` are appended to the loopback defaults of the inbound
/// `Host` check, which guards locally bound servers against DNS rebinding.
fn router(db_path: PathBuf, extra_allowed_hosts: Vec<String>) -> Router {
    let mut config = StreamableHttpServerConfig::default();
    config.allowed_hosts.extend(extra_allowed_hosts);

    let service = StreamableHttpService::new(
        move || Ok(LatServer::new(db_path.clone())),
        LocalSessionManager::default().into(),
        config,
    );

    Router::new()
        .route("/health", get(health))
        .nest_service(MCP_PATH, service)
}

/// Liveness probe.
async fn health() -> &'static str {
    "ok"
}

/// Serves the MCP endpoint over HTTP until the process is asked to shut down.
pub async fn serve(
    db_path: PathBuf,
    addr: SocketAddr,
    extra_allowed_hosts: Vec<String>,
) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("could not bind {addr}"))?;
    let bound = listener.local_addr().unwrap_or(addr);
    tracing::info!("lat MCP endpoint on http://{bound}{MCP_PATH}");

    axum::serve(listener, router(db_path, extra_allowed_hosts))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("HTTP server error")
}

/// Resolves on Ctrl-C. If the signal handler cannot be installed, the future
/// never resolves — the server then runs until the process is killed, rather
/// than shutting down immediately.
async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::warn!("could not install shutdown handler: {error}");
        std::future::pending::<()>().await;
    }
    tracing::info!("shutdown signal received");
}
