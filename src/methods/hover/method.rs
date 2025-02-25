use std::fs::File;
use std::io::{BufRead, BufReader};

use tower_lsp::jsonrpc::{Error, ErrorCode};
use tower_lsp::lsp_types::{
    Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Position, Range,
};
use urlencoding::decode;

use crate::methods::errors::NO_FILE_OR_DIRECTORY;

use super::texts::get_hover_text;

pub fn hover_method(params: HoverParams) -> Result<Option<Hover>, Error> {
    let file_path = params
        .text_document_position_params
        .text_document
        .uri
        .path();
    let position = params.text_document_position_params.position;

    let decoded_path = decode(file_path).expect("UTF-8");
    match File::open(decoded_path.to_string()) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut word = String::new();
            let mut word_line: u32 = 0;
            let mut word_range: (u32, u32) = (0, 0);

            for (line_index, line) in reader.lines().enumerate() {
                let line_position = position.line as usize;
                if line_index > line_position {
                    break;
                }
                if line_index < line_position {
                    continue;
                }
                let cursor_column = position.character as usize;
                let line_content = line.unwrap();
                let (word_in_code, start_col, end_col) =
                    get_word_in_line_col_position(line_content, cursor_column);
                word_line = line_position as u32;
                word_range = (start_col as u32, end_col as u32);
                word.push_str(word_in_code.as_str())
            }

            match get_hover_text(word) {
                Some(hover_text_content) => {
                    let result = Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: hover_text_content,
                        }),
                        range: Some(Range {
                            start: Position {
                                line: word_line,
                                character: word_range.0,
                            },
                            end: Position {
                                line: word_line,
                                character: word_range.1,
                            },
                        }),
                    };
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

fn get_word_in_line_col_position(
    line: String,
    cursor_column_position: usize,
) -> (String, usize, usize) {
    let line_len = line.len();

    if cursor_column_position > line_len {
        return ("".to_string(), 0, 0);
    }

    let mut start_word = 0;
    let mut end_word = line_len;

    for col_index in (0..cursor_column_position).rev() {
        let current_char = line.chars().nth(col_index).unwrap();
        if current_char.is_whitespace() {
            start_word = col_index + 1;
            break;
        } else {
            start_word = col_index;
        }
    }

    for col_index in cursor_column_position..line_len {
        let current_char = line.chars().nth(col_index).unwrap();
        if current_char.is_whitespace() {
            end_word = col_index;
            break;
        }
    }

    if start_word >= line_len || start_word >= end_word {
        return ("".to_string(), 0, 0);
    }

    let word = line.get(start_word..end_word).unwrap();
    (word.to_string(), start_word, end_word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_empty_when_cursor_is_bigger_than_line_len() {
        let line = "severo value = 10".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line.clone(), line.len() + 1);
        let expected_word = "".to_string();
        assert_eq!(expected_word, found_word);
    }

    #[test]
    fn should_found_word_when_cursor_is_in_start_of_word() {
        let line = "severo value = 10".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line, 0);
        let expected_word = "severo".to_string();
        assert_eq!(expected_word, found_word);
    }

    #[test]
    fn should_found_word_when_cursor_is_between_start_and_end_of_word() {
        let line = "severo value = 10".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line, 3);
        let expected_word = "severo".to_string();
        assert_eq!(expected_word, found_word);
    }

    #[test]
    fn should_found_word_when_cursor_is_in_end_of_word() {
        let line = "severo value = 10".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line, 6);
        let expected_word = "severo".to_string();
        assert_eq!(expected_word, found_word);
    }

    #[test]
    fn should_found_word_when_word_is_forward_from_start() {
        let line = "severo value = 10".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line, 12);
        let expected_word = "value".to_string();
        assert_eq!(expected_word, found_word);
    }

    #[test]
    fn should_found_word_when_is_the_last_in_line() {
        let line = "severo sum = fun".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line, 16);
        let expected_word = "fun".to_string();
        assert_eq!(expected_word, found_word);
    }
}
