// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! lat — Language-as-Tool MCP server.
//!
//! Exposes curated thinking patterns (languages and poetic forms) as read-only
//! MCP tools, either over stdio (default) or over streamable HTTP. Logs go to
//! stderr so that stdout carries the MCP protocol alone.

mod db;
mod http;
mod models;
mod server;

use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::EnvFilter;

use crate::server::LatServer;

/// Address used by `--http` when neither an argument nor `LAT_HTTP_ADDR` is
/// given. Loopback only, so nothing is exposed to the network by accident.
const DEFAULT_HTTP_ADDR: &str = "127.0.0.1:8000";

const USAGE: &str = "\
lat — Language-as-Tool MCP server

Usage:
  lat                     Serve over stdio (default; host launches the binary)
  lat --http [ADDR]       Serve over streamable HTTP at ADDR/mcp
  lat --help, -h          Show this help
  lat --version, -V       Show the version

Environment:
  LAT_DB_PATH             Database file (default: platform data directory)
  LAT_HTTP_ADDR           Address for --http (default: 127.0.0.1:8000)
  LAT_HTTP_ALLOWED_HOSTS  Additional accepted Host header values for HTTP,
                          comma-separated. Loopback is always accepted.
  RUST_LOG                Log filter (default: info)";

/// How the server talks to its host.
#[derive(Debug, Clone, Copy)]
enum Transport {
    Stdio,
    Http(SocketAddr),
}

/// What the invocation asked for.
#[derive(Debug, Clone, Copy)]
enum Command {
    Serve(Transport),
    Help,
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let command = match parse_args(std::env::args().skip(1)) {
        Ok(command) => command,
        Err(error) => {
            eprintln!("error: {error}\n\n{USAGE}");
            std::process::exit(2);
        }
    };

    match command {
        Command::Help => {
            println!("{USAGE}");
            Ok(())
        }
        Command::Version => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::Serve(transport) => run(transport).await,
    }
}

/// Prepares the database and serves on the chosen transport.
async fn run(transport: Transport) -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let db_path = db::init()?;
    tracing::info!("lat MCP server starting (db: {})", db_path.display());

    match transport {
        Transport::Stdio => serve_stdio(db_path).await,
        Transport::Http(addr) => http::serve(db_path, addr, extra_allowed_hosts()).await,
    }
}

/// Serves a single session over stdin/stdout.
async fn serve_stdio(db_path: PathBuf) -> Result<()> {
    let service = LatServer::new(db_path)
        .serve(stdio())
        .await
        .inspect_err(|error| tracing::error!("serve error: {error:?}"))?;

    service.waiting().await?;
    Ok(())
}

/// Parses the command line. Accepts `--http`, `--http ADDR` and `--http=ADDR`.
fn parse_args(args: impl Iterator<Item = String>) -> Result<Command> {
    let mut args = args.peekable();

    let command = match args.next() {
        None => Command::Serve(Transport::Stdio),
        Some(arg) => match arg.as_str() {
            "--help" | "-h" => Command::Help,
            "--version" | "-V" => Command::Version,
            "--http" => {
                // Only consume the next argument when it is a value, not a flag.
                let value = args
                    .next_if(|next| !next.starts_with('-'))
                    .map(|value| parse_addr(&value))
                    .transpose()?;
                Command::Serve(Transport::Http(match value {
                    Some(addr) => addr,
                    None => default_http_addr()?,
                }))
            }
            _ => match arg.strip_prefix("--http=") {
                Some(value) => Command::Serve(Transport::Http(parse_addr(value)?)),
                None => bail!("unknown argument '{arg}'"),
            },
        },
    };

    if let Some(extra) = args.next() {
        bail!("unexpected argument '{extra}'");
    }
    Ok(command)
}

/// Resolves the fallback HTTP address from `LAT_HTTP_ADDR`.
fn default_http_addr() -> Result<SocketAddr> {
    let raw = std::env::var("LAT_HTTP_ADDR").unwrap_or_else(|_| DEFAULT_HTTP_ADDR.to_owned());
    parse_addr(&raw).context("invalid LAT_HTTP_ADDR")
}

/// Resolves `host:port` (or `ip:port`) to a single socket address.
fn parse_addr(raw: &str) -> Result<SocketAddr> {
    let raw = raw.trim();
    if raw.is_empty() {
        bail!("empty address; expected host:port");
    }
    raw.to_socket_addrs()
        .with_context(|| format!("could not resolve address '{raw}'; expected host:port"))?
        .next()
        .with_context(|| format!("address '{raw}' resolved to no endpoint"))
}

/// Reads additional accepted `Host` values from `LAT_HTTP_ALLOWED_HOSTS`.
fn extra_allowed_hosts() -> Vec<String> {
    parse_allowed_hosts(&std::env::var("LAT_HTTP_ALLOWED_HOSTS").unwrap_or_default())
}

/// Splits a comma-separated host list, trimming blanks and dropping empties.
fn parse_allowed_hosts(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|host| !host.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Command> {
        parse_args(args.iter().map(|arg| (*arg).to_string()))
    }

    #[test]
    fn no_arguments_serves_over_stdio() {
        assert!(matches!(
            parse(&[]).unwrap(),
            Command::Serve(Transport::Stdio)
        ));
    }

    #[test]
    fn http_with_address_uses_that_address() {
        let Command::Serve(Transport::Http(addr)) = parse(&["--http", "127.0.0.1:9123"]).unwrap()
        else {
            panic!("expected an HTTP transport");
        };
        assert_eq!(addr.to_string(), "127.0.0.1:9123");
    }

    #[test]
    fn http_accepts_the_equals_form() {
        let Command::Serve(Transport::Http(addr)) = parse(&["--http=127.0.0.1:9124"]).unwrap()
        else {
            panic!("expected an HTTP transport");
        };
        assert_eq!(addr.to_string(), "127.0.0.1:9124");
    }

    #[test]
    fn unknown_argument_is_rejected() {
        assert!(parse(&["--nope"]).is_err());
    }

    #[test]
    fn trailing_argument_is_rejected() {
        assert!(parse(&["--http", "127.0.0.1:9125", "extra"]).is_err());
    }

    #[test]
    fn malformed_address_is_rejected() {
        assert!(parse(&["--http", "127.0.0.1"]).is_err());
    }

    #[test]
    fn help_and_version_are_recognised_in_both_spellings() {
        assert!(matches!(parse(&["--help"]).unwrap(), Command::Help));
        assert!(matches!(parse(&["-h"]).unwrap(), Command::Help));
        assert!(matches!(parse(&["--version"]).unwrap(), Command::Version));
        assert!(matches!(parse(&["-V"]).unwrap(), Command::Version));
    }

    #[test]
    fn the_usage_text_lists_every_accepted_flag_and_variable() {
        for token in [
            "--http",
            "--help",
            "-h",
            "--version",
            "-V",
            "LAT_DB_PATH",
            "LAT_HTTP_ADDR",
            "LAT_HTTP_ALLOWED_HOSTS",
            "RUST_LOG",
        ] {
            assert!(USAGE.contains(token), "usage does not mention {token}");
        }
    }

    #[test]
    fn http_does_not_swallow_a_following_flag() {
        // `lat --http --help` must not read `--help` as an address; the flag
        // then remains and is reported as unexpected.
        assert!(parse(&["--http", "--help"]).is_err());
    }

    #[test]
    fn an_empty_address_is_rejected() {
        assert!(parse(&["--http="]).is_err());
        assert!(parse(&["--http", "   "]).is_err());
    }

    #[test]
    fn an_address_may_be_padded_with_whitespace() {
        let Command::Serve(Transport::Http(addr)) = parse(&["--http", " 127.0.0.1:9126 "]).unwrap()
        else {
            panic!("expected an HTTP transport");
        };
        assert_eq!(addr.to_string(), "127.0.0.1:9126");
    }

    #[test]
    fn a_host_name_is_resolved_to_an_endpoint() {
        let Command::Serve(Transport::Http(addr)) = parse(&["--http", "localhost:9127"]).unwrap()
        else {
            panic!("expected an HTTP transport");
        };
        assert!(addr.ip().is_loopback(), "localhost should be loopback");
        assert_eq!(addr.port(), 9127);
    }

    #[test]
    fn an_unresolvable_address_is_rejected() {
        assert!(parse_addr("no-such-host.invalid:80").is_err());
        assert!(parse_addr("127.0.0.1:not-a-port").is_err());
    }

    #[test]
    fn an_ipv6_address_is_accepted_in_bracket_form() {
        let addr = parse_addr("[::1]:9128").unwrap();
        assert!(addr.ip().is_loopback());
        assert_eq!(addr.port(), 9128);
    }

    #[test]
    fn the_default_http_address_is_loopback_only() {
        // Nothing should reach the network by accident when `--http` is given
        // without an address.
        let addr = parse_addr(DEFAULT_HTTP_ADDR).unwrap();
        assert!(
            addr.ip().is_loopback(),
            "the default address must not be routable"
        );
    }

    #[test]
    fn allowed_hosts_are_split_trimmed_and_cleaned() {
        assert_eq!(
            parse_allowed_hosts("lat.internal, box.local"),
            vec!["lat.internal", "box.local"]
        );
        assert_eq!(parse_allowed_hosts("  spaced  "), vec!["spaced"]);
    }

    #[test]
    fn an_empty_allowed_hosts_list_widens_nothing() {
        assert!(parse_allowed_hosts("").is_empty());
        assert!(parse_allowed_hosts(",").is_empty());
        assert!(parse_allowed_hosts("  ,  , ").is_empty());
    }
}
