// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Dana Schlifka

//! End-to-end test of the stdio transport.
//!
//! Launches the built binary the way an MCP host does — as a child process
//! speaking line-delimited JSON-RPC over stdin/stdout — and exercises the
//! handshake and all four tools. This is the only place that covers the wiring
//! between `LAT_DB_PATH`, seeding on first start and the tool router.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::Duration;

use serde_json::{Value, json};

/// How long to wait for a single response before declaring the server stuck.
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

/// Protocol version the test client negotiates.
const PROTOCOL_VERSION: &str = "2024-11-05";

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A running `lat` child process with its own database directory.
struct Host {
    child: Child,
    stdin: ChildStdin,
    lines: Receiver<String>,
    dir: PathBuf,
    db_path: PathBuf,
    next_id: i64,
}

impl Host {
    /// Starts the binary with `LAT_DB_PATH` inside a fresh temporary directory
    /// that does not exist yet, so first-start creation is covered too.
    fn start() -> Self {
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("lat-stdio-{}-{unique}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        // Deliberately nested: db_path() has to create the missing parent.
        let db_path = dir.join("nested").join("patterns.db");

        let mut child = Command::new(env!("CARGO_BIN_EXE_lat"))
            .env("LAT_DB_PATH", &db_path)
            .env("RUST_LOG", "error")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("could not start the lat binary");

        let stdin = child.stdin.take().expect("no stdin");
        let stdout = child.stdout.take().expect("no stdout");

        // Read on a worker thread so a stuck server turns into a timeout
        // rather than a hanging test.
        let (sender, lines) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(line) => {
                        if sender.send(line).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Self {
            child,
            stdin,
            lines,
            dir,
            db_path,
            next_id: 0,
        }
    }

    fn send(&mut self, message: &Value) {
        writeln!(self.stdin, "{message}").expect("could not write to the server");
        self.stdin.flush().expect("could not flush stdin");
    }

    fn next_line(&self) -> String {
        match self.lines.recv_timeout(RESPONSE_TIMEOUT) {
            Ok(line) => line,
            Err(RecvTimeoutError::Timeout) => panic!("the server did not answer in time"),
            Err(RecvTimeoutError::Disconnected) => panic!("the server closed its output"),
        }
    }

    /// Sends a request and returns the response with the matching id,
    /// skipping any notification the server emits in between.
    fn request(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = self.next_id;
        self.send(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }));

        loop {
            let line = self.next_line();
            let message: Value = serde_json::from_str(&line)
                .unwrap_or_else(|error| panic!("not JSON ({error}): {line}"));
            if message.get("id").and_then(Value::as_i64) == Some(id) {
                return message;
            }
        }
    }

    /// Sends a request and returns its `result`, failing on a JSON-RPC error.
    fn call(&mut self, method: &str, params: Value) -> Value {
        let message = self.request(method, params);
        assert!(
            message.get("error").is_none(),
            "{method} failed: {}",
            message["error"]
        );
        message["result"].clone()
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.send(&json!({"jsonrpc": "2.0", "method": method, "params": params}));
    }

    /// Performs the MCP handshake and returns the `initialize` result.
    fn initialize(&mut self) -> Value {
        let result = self.call(
            "initialize",
            json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": {"name": "lat-integration-test", "version": "0"},
            }),
        );
        self.notify("notifications/initialized", json!({}));
        result
    }

    /// Calls a tool and returns the parsed JSON of its text content.
    fn call_tool(&mut self, name: &str, arguments: Value) -> Value {
        let result = self.call("tools/call", json!({"name": name, "arguments": arguments}));
        assert_ne!(
            result.get("isError").and_then(Value::as_bool),
            Some(true),
            "{name} reported an error: {result}"
        );
        serde_json::from_str(&tool_text(&result))
            .unwrap_or_else(|error| panic!("{name} did not return JSON ({error}): {result}"))
    }

    fn db_path(&self) -> &Path {
        &self.db_path
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        // Closing stdin ends the session; kill anything that lingers.
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// Concatenates the text blocks of a `tools/call` result.
fn tool_text(result: &Value) -> String {
    result["content"]
        .as_array()
        .expect("a tool result should carry content")
        .iter()
        .filter(|block| block["type"] == "text")
        .map(|block| block["text"].as_str().expect("text should be a string"))
        .collect::<Vec<_>>()
        .join("")
}

fn names(patterns: &Value) -> Vec<String> {
    patterns
        .as_array()
        .expect("expected an array of patterns")
        .iter()
        .map(|pattern| pattern["name"].as_str().expect("missing name").to_owned())
        .collect()
}

#[test]
fn the_handshake_announces_the_server_and_its_tools() {
    let mut host = Host::start();
    let result = host.initialize();

    assert_eq!(result["protocolVersion"], PROTOCOL_VERSION);
    assert_eq!(result["serverInfo"]["name"], env!("CARGO_PKG_NAME"));
    assert_eq!(result["serverInfo"]["version"], env!("CARGO_PKG_VERSION"));
    assert!(
        result["capabilities"]["tools"].is_object(),
        "tools should be advertised: {result}"
    );
    assert!(
        result["instructions"]
            .as_str()
            .is_some_and(|text| text.contains("search_patterns")),
        "the instructions should point at the tools: {result}"
    );
}

#[test]
fn the_first_start_creates_and_seeds_the_database_at_lat_db_path() {
    let mut host = Host::start();
    host.initialize();

    let patterns = host.call_tool("list_patterns", json!({}));
    assert!(!names(&patterns).is_empty(), "the seed should be applied");
    assert!(
        host.db_path().exists(),
        "LAT_DB_PATH should have been created, including its parent directory"
    );
}

#[test]
fn all_four_tools_are_listed_with_a_schema() {
    let mut host = Host::start();
    host.initialize();

    let result = host.call("tools/list", json!({}));
    let tools = result["tools"].as_array().expect("expected a tool array");

    let mut listed: Vec<&str> = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("a tool needs a name"))
        .collect();
    listed.sort_unstable();
    assert_eq!(
        listed,
        [
            "get_pattern",
            "list_facets",
            "list_patterns",
            "search_patterns"
        ]
    );

    for tool in tools {
        let name = tool["name"].as_str().unwrap();
        assert!(
            tool["description"]
                .as_str()
                .is_some_and(|text| !text.is_empty()),
            "{name} has no description"
        );
        assert_eq!(
            tool["inputSchema"]["type"], "object",
            "{name} has no object schema"
        );
    }

    let get_pattern = tools
        .iter()
        .find(|tool| tool["name"] == "get_pattern")
        .expect("get_pattern should be listed");
    let required = get_pattern["inputSchema"]["required"]
        .as_array()
        .expect("get_pattern should require parameters");
    assert!(required.iter().any(|field| field == "kind"));
    assert!(required.iter().any(|field| field == "name"));
}

#[test]
fn list_facets_advertises_filters_that_search_patterns_accepts() {
    let mut host = Host::start();
    host.initialize();

    let facets = host.call_tool("list_facets", json!({}));
    let entries = facets.as_array().expect("expected one entry per kind");
    assert_eq!(entries.len(), 2);

    for entry in entries {
        let kind = entry["kind"].as_str().expect("a facet entry needs a kind");
        let theme = entry["themes"][0]
            .as_str()
            .expect("every kind should offer at least one theme");

        let found = host.call_tool("search_patterns", json!({"kind": kind, "theme": theme}));
        assert!(
            !names(&found).is_empty(),
            "theme '{theme}' is advertised for {kind} but matches nothing"
        );

        let attachment = entry["attachments"][0]
            .as_str()
            .expect("every kind should offer at least one attachment");
        let anchored = host.call_tool(
            "search_patterns",
            json!({"kind": kind, "attachment": attachment}),
        );
        assert!(
            !names(&anchored).is_empty(),
            "attachment '{attachment}' is advertised for {kind} but matches nothing"
        );
    }
}

#[test]
fn patterns_forcing_one_choice_are_found_together_by_that_choice() {
    // What the two combination columns are for: Tuyuca and Tariana force the
    // same choice at the same anchor, so a caller can see the collision by
    // filtering rather than by reading the prose columns.
    let mut host = Host::start();
    host.initialize();

    let tuyuca = host.call_tool("get_pattern", json!({"kind": "language", "name": "Tuyuca"}));
    let choice = tuyuca["forced_choice"]
        .as_str()
        .expect("a seeded pattern names the choice it forces");
    assert_eq!(tuyuca["attachment"], "verb");

    let same = host.call_tool(
        "search_patterns",
        json!({"kind": "language", "forced_choice": choice, "attachment": "verb"}),
    );
    let found = names(&same);
    assert!(
        found.contains(&"Tuyuca".to_owned()) && found.contains(&"Tariana".to_owned()),
        "expected Tuyuca and Tariana to share the choice, got {found:?}"
    );
}

#[test]
fn get_pattern_returns_a_full_record_and_reports_a_miss() {
    let mut host = Host::start();
    host.initialize();

    let german = host.call_tool("get_pattern", json!({"kind": "language", "name": "German"}));
    assert_eq!(german["kind"], "language");
    assert_eq!(german["name"], "German");
    assert!(german["tags"].is_array());
    assert!(german["themes"].is_array());
    assert!(german["forced_choice"].is_string());
    assert!(german["attachment"].is_string());

    // A miss is an answer, not a protocol error, so the text is not JSON.
    let result = host.call(
        "tools/call",
        json!({
            "name": "get_pattern",
            "arguments": {"kind": "form", "name": "No Such Form"},
        }),
    );
    assert_ne!(result.get("isError").and_then(Value::as_bool), Some(true));
    assert!(tool_text(&result).contains("No Such Form"));
}

#[test]
fn search_patterns_filters_and_excludes_the_source_language() {
    let mut host = Host::start();
    host.initialize();

    let all = names(&host.call_tool("search_patterns", json!({"kind": "language"})));
    assert!(all.contains(&"German".to_owned()));

    let contrasting = names(&host.call_tool(
        "search_patterns",
        json!({"kind": "language", "exclude_names": ["German"]}),
    ));
    assert!(!contrasting.contains(&"German".to_owned()));
    assert_eq!(contrasting.len(), all.len() - 1);

    let empty = host.call_tool("search_patterns", json!({"tag": "no-such-tag-exists"}));
    assert_eq!(empty, json!([]));
}

#[test]
fn an_unknown_tool_is_answered_with_an_error_rather_than_a_crash() {
    let mut host = Host::start();
    host.initialize();

    let message = host.request(
        "tools/call",
        json!({"name": "no_such_tool", "arguments": {}}),
    );
    let failed = message.get("error").is_some()
        || message["result"].get("isError").and_then(Value::as_bool) == Some(true);
    assert!(failed, "an unknown tool should fail: {message}");

    // The session must survive it.
    let patterns = host.call_tool("list_patterns", json!({"kind": "form"}));
    assert!(!names(&patterns).is_empty());
}

#[test]
fn invalid_tool_arguments_are_rejected_without_ending_the_session() {
    let mut host = Host::start();
    host.initialize();

    let message = host.request(
        "tools/call",
        json!({"name": "get_pattern", "arguments": {"kind": "dialect", "name": "German"}}),
    );
    let failed = message.get("error").is_some()
        || message["result"].get("isError").and_then(Value::as_bool) == Some(true);
    assert!(failed, "'dialect' is not a valid kind: {message}");

    let patterns = host.call_tool("list_patterns", json!({}));
    assert!(!names(&patterns).is_empty());
}

#[test]
fn help_and_version_are_printed_without_starting_a_server() {
    let help = Command::new(env!("CARGO_BIN_EXE_lat"))
        .arg("--help")
        .output()
        .expect("could not run the binary");
    assert!(help.status.success());
    let text = String::from_utf8_lossy(&help.stdout);
    assert!(text.contains("Usage:"), "unexpected help output: {text}");

    let version = Command::new(env!("CARGO_BIN_EXE_lat"))
        .arg("--version")
        .output()
        .expect("could not run the binary");
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).trim(),
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn an_unknown_argument_exits_with_a_usage_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_lat"))
        .arg("--nope")
        .output()
        .expect("could not run the binary");

    assert_eq!(
        output.status.code(),
        Some(2),
        "expected the usage exit code"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--nope"), "unexpected stderr: {stderr}");
    assert!(stderr.contains("Usage:"), "usage should be shown: {stderr}");
}
