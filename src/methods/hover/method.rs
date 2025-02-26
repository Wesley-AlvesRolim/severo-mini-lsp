use std::fs::File;
use std::io::{BufRead, BufReader};

use tower_lsp::jsonrpc::{Error, ErrorCode};
use tower_lsp::lsp_types::{
    Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Position, Range,
};
use urlencoding::decode;

use crate::helpers::get_word_in_line_col_position::get_word_in_line_col_position;
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
                if line_index < line_position {
                    continue;
                }
                let cursor_column = position.character as usize;
                let line_content = line.unwrap();
                let (word_in_code, start_col, end_col) =
                    get_word_in_line_col_position(line_content, cursor_column);
                word_line = line_position as u32;
                word_range = (start_col as u32, end_col as u32);
                word.push_str(word_in_code.as_str());
                break;
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
