// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! lat — Language-as-Tool MCP server.
//!
//! Exposes curated thinking patterns (languages and poetic forms) as read-only
//! MCP tools over stdio. Logs go to stderr so that stdout carries the MCP
//! protocol alone.

mod db;
mod models;
mod server;

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::EnvFilter;

use crate::server::LatServer;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let db_path = db::init()?;
    tracing::info!("lat MCP server starting (db: {})", db_path.display());

    let service = LatServer::new(db_path)
        .serve(stdio())
        .await
        .inspect_err(|error| tracing::error!("serve error: {error:?}"))?;

    service.waiting().await?;
    Ok(())
}
