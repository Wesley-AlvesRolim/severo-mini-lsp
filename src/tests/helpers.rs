use std::env;

use serde_json::{json, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
use tower_lsp::{
    jsonrpc::Request,
    lsp_types::{ClientCapabilities, Url},
    LspService, Server,
};

use tokio::io::duplex;

use crate::backend::Backend;

pub fn create_lsp() -> (DuplexStream, DuplexStream) {
    let (req_client, req_server) = duplex(1024);
    let (res_server, res_client) = duplex(1024);

    let (service, socket) = LspService::new(|client| Backend { client });
    tokio::spawn(Server::new(req_server, res_server, socket).serve(service));
    (req_client, res_client)
}

pub async fn init_lsp() -> (DuplexStream, DuplexStream) {
    let (mut req_client, mut resp_client) = create_lsp();
    let initialize_request_string = format_request(initialize_request(0));
    req_client
        .write_all(initialize_request_string.as_bytes())
        .await
        .unwrap();

    let mut buf = vec![0; 1024];
    let _ = resp_client.read(&mut buf).await.unwrap();
    println!("1: {}", String::from_utf8_lossy(buf.as_slice()));

    (req_client, resp_client)
}

pub async fn get_response_string(mut resp_client: DuplexStream) -> String {
    let mut buf = vec![0; 1024];
    let bytes = resp_client.read(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf[0..bytes]).to_string()
}

pub fn format_request(request: Request) -> String {
    let request_str = request.to_string();
    format!(
        "Content-Length: {}\r\n\r\n{}",
        request_str.len(),
        request_str
    )
}

pub fn initialize_request(id: i64) -> Request {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let hover_mock = current_dir
        .join("tests/mocks/hover.severo")
        .to_string_lossy()
        .to_string();
    Request::build("initialize")
        .params(json!({
            "process_id": std::process::id(),
            "root_uri": Url::parse(format!("file://{}", hover_mock).as_str()).unwrap(),
            "capabilities":ClientCapabilities::default(),
            "trace": "off",
        }))
        .id(id)
        .finish()
}

pub fn initialized_request(id: i64) -> Request {
    Request::build("initialized").id(id).finish()
}

pub fn hover_request(id: i64, file_uri: String, line: usize, character: usize) -> Request {
    let uri_formatted = format!("file://{}", file_uri);
    Request::build("textDocument/hover")
        .id(id)
        .params(json!({
            "textDocument": {"uri": uri_formatted},
            "position": {"line": line, "character": character}
        }))
        .finish()
}

pub fn completion_request(id: i64, file_uri: String, line: usize, character: usize) -> Request {
    let uri_formatted = format!("file://{}", file_uri);
    Request::build("textDocument/completion")
        .id(id)
        .params(json!({
            "textDocument": {"uri": uri_formatted},
            "position": {"line": line, "character": character}
        }))
        .finish()
}

pub fn shutdown_request(id: i64) -> Request {
    Request::build("shutdown").id(id).finish()
}

pub fn format_response(response: Value) -> String {
    let request_str = response.to_string();
    format!(
        "Content-Length: {}\r\n\r\n{}",
        request_str.len(),
        request_str
    )
}

pub fn build_response(id: i64, body: Result<Value, Value>) -> Value {
    match body {
        Ok(body) => {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": body
            })
        }
        Err(body) => {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": body
            })
        }
    }
}

pub fn assert_outputs(left: String, right: String) {
    println!("LEFT:\n{}\nRIGHT:\n{}", left, right);
    let separator = "\r\n\r\n";
    let left_parts: Vec<&str> = left.split(separator).collect();
    let right_parts: Vec<&str> = right.split(separator).collect();

    assert_eq!(
        left_parts.first(),
        right_parts.first(),
        "Headers do not match"
    );

    let left_body = left_parts.get(1).expect("Left body is missing.");
    let right_body = right_parts.get(1).expect("Right body is missing.");

    let left_json: Value = serde_json::from_str(left_body).expect("Invalid JSON in left body");
    let right_json: Value = serde_json::from_str(right_body).expect("Invalid JSON in right body");

    assert_eq!(left_json, right_json, "JSON bodies do not match");
}
