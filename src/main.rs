use backend::Backend;
use tower_lsp::{LspService, Server};

pub mod backend;
pub mod consts;
pub mod methods;
pub mod tests;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
