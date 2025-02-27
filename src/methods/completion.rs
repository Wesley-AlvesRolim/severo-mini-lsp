use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use tower_lsp::{
    jsonrpc::{Error, ErrorCode},
    lsp_types::{CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse},
};
use urlencoding::decode;

use crate::{
    helpers::get_word_in_line_col_position::get_word_in_line_col_position,
    methods::errors::NO_FILE_OR_DIRECTORY,
    spec::{
        builtin_functions::get_builtin_functions, keywords::get_keywords, parser::get_vars,
        scanner::scan_tokens,
    },
};

pub fn completion_method(params: CompletionParams) -> Result<Option<CompletionResponse>, Error> {
    let file_path = params.text_document_position.text_document.uri.path();
    let position = params.text_document_position.position;

    let decoded_path = decode(file_path).expect("UTF-8");
    match File::open(decoded_path.to_string()) {
        Ok(file) => {
            let mut source = String::new();
            let reader = BufReader::new(file);
            for (line_index, line) in reader.lines().enumerate() {
                let line_position = position.line as usize;
                if line_index > line_position {
                    break;
                }
                if let Ok(line_content) = line {
                    source.push_str(line_content.as_str());
                    source.push('\n');
                }
            }

            let scan_result = scan_tokens(source);
            let variables = get_vars(scan_result.tokens);

            let file = File::open(decoded_path.to_string()).unwrap();
            let reader = BufReader::new(file);
            let mut word_or_part_of_it = String::new();
            for (line_index, line) in reader.lines().enumerate() {
                let line_position = position.line as usize;
                if line_index < line_position {
                    continue;
                }
                let cursor_column = position.character as usize;
                let line_content = line.unwrap();
                let (word_in_code, _, _) =
                    get_word_in_line_col_position(line_content, cursor_column);
                word_or_part_of_it.push_str(word_in_code.as_str());
                break;
            }
            match get_completion_items(word_or_part_of_it, variables) {
                Some(completion_items) => {
                    let result = CompletionResponse::Array(completion_items);
                    Ok(Some(result))
                }
                None => Ok(None),
            }
        }
        Err(_) => Err(Error {
            code: ErrorCode::InvalidParams,
            message: String::from_utf8_lossy(NO_FILE_OR_DIRECTORY.as_bytes()),
            data: None,
        }),
    }
}

fn get_completion_items(
    word_or_part_of_it: String,
    variables: Vec<String>,
) -> Option<Vec<CompletionItem>> {
    let mut completion_items: Vec<CompletionItem> = Vec::new();

    for keyword in get_keywords() {
        if keyword.starts_with(word_or_part_of_it.as_str()) {
            completion_items.push(CompletionItem {
                label: keyword,
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }
    }

    for builtin_function in get_builtin_functions() {
        if builtin_function.starts_with(word_or_part_of_it.as_str()) {
            completion_items.push(CompletionItem {
                label: builtin_function,
                kind: Some(CompletionItemKind::FUNCTION),
                ..Default::default()
            });
        }
    }

    for variable in variables {
        if variable.starts_with(word_or_part_of_it.as_str()) {
            completion_items.push(CompletionItem {
                label: variable,
                kind: Some(CompletionItemKind::VARIABLE),
                ..Default::default()
            });
        }
    }

    if completion_items.is_empty() {
        None
    } else {
        Some(completion_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_found_a_keyword_for_completion() {
        let word = "sev".to_string();
        let found_completion_items = get_completion_items(word.clone(), Vec::new());
        let expected_completion_items = vec![CompletionItem {
            label: "severo".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            ..Default::default()
        }];
        assert_eq!(Some(expected_completion_items), found_completion_items);
    }

    #[test]
    fn should_found_builtin_function_for_completion() {
        let word = "prin".to_string();
        let found_completion_items = get_completion_items(word.clone(), Vec::new());
        let expected_completion_items = vec![CompletionItem {
            label: "print".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        }];
        assert_eq!(Some(expected_completion_items), found_completion_items);
    }

    #[test]
    fn should_found_variables_for_completion() {
        let variable_name = "variableName";
        let variables = vec![variable_name.to_string()];
        let word = "var".to_string();
        let found_completion_items = get_completion_items(word.clone(), variables);
        let expected_completion_items = vec![CompletionItem {
            label: variable_name.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            ..Default::default()
        }];
        assert_eq!(Some(expected_completion_items), found_completion_items);
    }

    #[test]
    fn should_return_completion_when_word_is_complete() {
        let word = "print".to_string();
        let found_completion_items = get_completion_items(word.clone(), Vec::new());
        let expected_completion_items = vec![CompletionItem {
            label: "print".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        }];
        assert_eq!(Some(expected_completion_items), found_completion_items);
    }

    #[test]
    fn should_return_empty() {
        let word = "invalid".to_string();
        let found_completion_items = get_completion_items(word.clone(), Vec::new());
        let expected_completion_items = None;
        assert_eq!(expected_completion_items, found_completion_items);
    }
}
