use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::{
    CompletionOptions, HoverProviderCapability, InitializeResult, ServerCapabilities, ServerInfo,
    WorkDoneProgressOptions,
};

use crate::consts::{SERVER_NAME, SERVER_VERSION};

pub fn initialize() -> Result<InitializeResult, Error> {
    Ok(InitializeResult {
        capabilities: ServerCapabilities {
            position_encoding: None,
            text_document_sync: None,
            selection_range_provider: None,
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            completion_provider: Some(CompletionOptions {
                resolve_provider: None,
                trigger_characters: None,
                all_commit_characters: None,
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: None,
                },
                completion_item: None,
            }),
            signature_help_provider: None,
            definition_provider: None,
            type_definition_provider: None,
            implementation_provider: None,
            references_provider: None,
            document_highlight_provider: None,
            document_symbol_provider: None,
            workspace_symbol_provider: None,
            code_action_provider: None,
            code_lens_provider: None,
            document_formatting_provider: None,
            document_range_formatting_provider: None,
            document_on_type_formatting_provider: None,
            rename_provider: None,
            document_link_provider: None,
            color_provider: None,
            folding_range_provider: None,
            declaration_provider: None,
            execute_command_provider: None,
            workspace: None,
            call_hierarchy_provider: None,
            semantic_tokens_provider: None,
            moniker_provider: None,
            linked_editing_range_provider: None,
            inline_value_provider: None,
            inlay_hint_provider: None,
            diagnostic_provider: None,
            experimental: None,
        },
        server_info: Some(ServerInfo {
            name: SERVER_NAME.to_string(),
            version: Some(SERVER_VERSION.to_string()),
        }),
    })
}
