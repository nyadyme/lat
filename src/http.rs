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

#[cfg(test)]
mod tests {
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode, header};
    use tower::ServiceExt;

    use super::*;
    use crate::db::testing::TempDb;

    /// Largest response body the tests will read.
    const BODY_LIMIT: usize = 64 * 1024;

    fn test_router(db: &TempDb, extra_allowed_hosts: Vec<String>) -> Router {
        router(db.path().to_path_buf(), extra_allowed_hosts)
    }

    /// A JSON-RPC `initialize` request against the MCP endpoint.
    fn initialize_request(host: &str) -> Request<Body> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "lat-tests", "version": "0"},
            },
        });

        Request::builder()
            .method("POST")
            .uri(MCP_PATH)
            .header(header::HOST, host)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json, text/event-stream")
            .body(Body::from(body.to_string()))
            .expect("could not build the request")
    }

    async fn body_string(response: axum::response::Response) -> String {
        let bytes = to_bytes(response.into_body(), BODY_LIMIT)
            .await
            .expect("could not read the body");
        String::from_utf8(bytes.to_vec()).expect("the body should be UTF-8")
    }

    #[tokio::test]
    async fn health_answers_ok() {
        let db = TempDb::new();
        let response = test_router(&db, Vec::new())
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_string(response).await, "ok");
    }

    #[tokio::test]
    async fn an_unknown_path_is_not_found() {
        let db = TempDb::new();
        let response = test_router(&db, Vec::new())
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn the_mcp_endpoint_is_mounted_and_initializes() {
        let db = TempDb::new();
        let response = test_router(&db, Vec::new())
            .oneshot(initialize_request("127.0.0.1"))
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "initialize over loopback should succeed"
        );

        // The streamable HTTP transport may answer as JSON or as a single SSE
        // event; both carry the same JSON-RPC payload.
        let body = body_string(response).await;
        assert!(
            body.contains("\"serverInfo\"") && body.contains(env!("CARGO_PKG_NAME")),
            "unexpected initialize response: {body}"
        );
    }

    #[tokio::test]
    async fn a_foreign_host_header_is_rejected() {
        // The Host check is what keeps a browser on another origin from
        // reaching the loopback server via DNS rebinding.
        let db = TempDb::new();
        let response = test_router(&db, Vec::new())
            .oneshot(initialize_request("evil.example"))
            .await
            .unwrap();

        assert_ne!(
            response.status(),
            StatusCode::OK,
            "a foreign Host must not be served"
        );
    }

    #[tokio::test]
    async fn an_explicitly_allowed_host_is_accepted() {
        let db = TempDb::new();
        let response = test_router(&db, vec!["lat.internal".to_owned()])
            .oneshot(initialize_request("lat.internal"))
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "LAT_HTTP_ALLOWED_HOSTS should widen the Host check"
        );
    }

    #[tokio::test]
    async fn the_health_probe_ignores_the_host_check() {
        // /health sits outside the MCP service, so a supervisor probing through
        // a proxy name still gets an answer.
        let db = TempDb::new();
        let response = test_router(&db, Vec::new())
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .header(header::HOST, "some.proxy.example")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
