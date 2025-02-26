use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::methods::completion::completion_method;
use crate::methods::hover::method::hover_method;
use crate::methods::initialize::initialize;

pub struct Backend {
    pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult, Error> {
        initialize()
    }

    async fn initialized(&self, _params: InitializedParams) {}

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>, Error> {
        hover_method(params)
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>, Error> {
        completion_method(params)
    }

    async fn shutdown(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use serde_json::json;
    use tokio::io::AsyncWriteExt;

    use crate::{
        consts::{SERVER_NAME, SERVER_VERSION},
        methods::{errors::NO_FILE_OR_DIRECTORY, hover::texts::VAR},
        tests::helpers::{
            assert_outputs, build_response, completion_request, create_lsp, format_request,
            format_response, get_response_string, hover_request, init_lsp, initialize_request,
            shutdown_request,
        },
    };

    #[tokio::test(flavor = "current_thread")]
    async fn should_initializes() {
        let request_id = 1;
        let expected_response = format_response(build_response(
            request_id,
            Ok(json!({
                "capabilities":{"hoverProvider":true,"completionProvider":{}},
                "serverInfo":{"name":SERVER_NAME,"version":SERVER_VERSION}
            })),
        ));
        let (mut req_client, resp_client) = create_lsp();
        let initialize_request = initialize_request(request_id);
        req_client
            .write_all(format_request(initialize_request).as_bytes())
            .await
            .unwrap();
        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_hover() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(
            request_id,
            Ok(json!({
                "contents":{
                    "kind":"markdown",
                    "value":VAR
                },
                "range":{
                    "end":{"character":6,"line":1},
                    "start":{"character":0,"line":1}
                }
            })),
        ));

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let hover_mock = current_dir
            .join("src/tests/mocks/hover.severo")
            .to_string_lossy()
            .to_string();
        let hover_request = hover_request(request_id, hover_mock, 1, 0);
        req_client
            .write_all(format_request(hover_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hover_error_with_path() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(
            request_id,
            Err(json!({
                "code":-32602,
                "message":NO_FILE_OR_DIRECTORY
            })),
        ));

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let hover_mock = current_dir
            .join("invalid_path.severo")
            .to_string_lossy()
            .to_string();
        let hover_request = hover_request(request_id, hover_mock, 0, 0);
        req_client
            .write_all(format_request(hover_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn hover_no_output_for_non_keywords() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(request_id, Ok(json!(null))));

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let hover_mock = current_dir
            .join("src/tests/mocks/hover.severo")
            .to_string_lossy()
            .to_string();
        let hover_request = hover_request(request_id, hover_mock, 1, 8);
        req_client
            .write_all(format_request(hover_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn completion() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(
            request_id,
            Ok(json!([
              {"label":"severo", "kind":14}
            ])),
        ));

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let completion_mock = current_dir
            .join("src/tests/mocks/completion.severo")
            .to_string_lossy()
            .to_string();
        let completion_request = completion_request(request_id, completion_mock, 0, 4);
        req_client
            .write_all(format_request(completion_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn completion_for_function() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(
            request_id,
            Ok(json!([
              {"label":"print", "kind":3}
            ])),
        ));

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let completion_mock = current_dir
            .join("src/tests/mocks/completion.severo")
            .to_string_lossy()
            .to_string();
        let completion_request = completion_request(request_id, completion_mock, 1, 5);
        req_client
            .write_all(format_request(completion_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn completion_empty() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(request_id, Ok(json!(null))));

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let completion_mock = current_dir
            .join("src/tests/mocks/completion.severo")
            .to_string_lossy()
            .to_string();
        let completion_request = completion_request(request_id, completion_mock, 2, 7);
        req_client
            .write_all(format_request(completion_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn completion_error_with_path() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(
            request_id,
            Err(json!({
                "code":-32602,
                "message":NO_FILE_OR_DIRECTORY
            })),
        ));

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let completion_mock = current_dir
            .join("invalid_path.severo")
            .to_string_lossy()
            .to_string();
        let completion_request = completion_request(request_id, completion_mock, 0, 0);
        req_client
            .write_all(format_request(completion_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn shutdown() {
        let (mut req_client, resp_client) = init_lsp().await;
        let request_id = 3;
        let expected_response = format_response(build_response(request_id, Ok(json!(null))));

        let shutdown_request = shutdown_request(request_id);
        req_client
            .write_all(format_request(shutdown_request).as_bytes())
            .await
            .unwrap();

        let response = get_response_string(resp_client).await;
        assert_outputs(expected_response, response)
    }
}
