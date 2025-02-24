use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

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

    async fn shutdown(&self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;
    use tokio::io::AsyncWriteExt;

    use crate::{
        consts::{SERVER_NAME, SERVER_VERSION},
        tests::helpers::{
            assert_outputs, build_response, create_lsp, format_request, format_response,
            get_response_string, initialize_request,
        },
    };

    #[tokio::test(flavor = "current_thread")]
    async fn initializes_only_once() {
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
}
