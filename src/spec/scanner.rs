use super::{
    helpers::{invert_escape, is_alpha, is_alphanumeric, is_numeric, rest_matches},
    keywords::get_keywords_hash,
    types::{Literal, ScanError, ScanResult, Token, TokenType},
};

pub fn scan_tokens(source: String) -> ScanResult {
    let mut tokens: Vec<Token> = Vec::new();
    for (line_count, line) in source.clone().lines().enumerate() {
        let line_tokens = scan_tokens_in_line(line.to_string(), line_count);
        tokens.extend(line_tokens);
    }
    tokens.push(Token {
        token_type: TokenType::Eof,
        line: 0,
        col: 0,
        literal: None,
    });
    ScanResult { tokens }
}

fn scan_tokens_in_line(line: String, line_count: usize) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut index = 0;
    let end_of_line = line.len();
    while index < end_of_line {
        if line.clone().get(index..index + 1).is_none() {
            break;
        }
        match line.clone().get(index..index + 1).unwrap() {
            "\r" | "\t" | " " | "\n" | "\0" => {
                index += 1;
                continue;
            }
            "(" => {
                tokens.push(Token {
                    token_type: TokenType::LeftParen,
                    line: line_count,
                    col: index,
                    literal: None,
                });
                index += 1;
                continue;
            }
            ")" => {
                tokens.push(Token {
                    token_type: TokenType::RightParen,
                    line: line_count,
                    col: index,
                    literal: None,
                });
                index += 1;
                continue;
            }
            "=" => match rest_matches(index, line.as_str(), "=") {
                true => {
                    index += 1;
                    continue;
                }
                false => {
                    tokens.push(Token {
                        token_type: TokenType::Equal,
                        line: line_count,
                        col: index,
                        literal: None,
                    });
                    index += 1;
                    continue;
                }
            },
            "\"" => match string(line.clone(), line_count, index) {
                Ok((token, skip)) => {
                    tokens.push(token);
                    index += skip;
                    continue;
                }
                Err(_) => {
                    index += 1;
                    continue;
                }
            },
            char => {
                if is_numeric(char) {
                    match number(line.clone(), line_count, index) {
                        Ok((token, skip)) => {
                            tokens.push(token);
                            index += skip;
                            continue;
                        }
                        Err(_) => {
                            index += 1;
                            continue;
                        }
                    }
                } else if is_alpha(char) {
                    let (token, skip) = identifier(line.clone(), line_count, index);
                    tokens.push(token);
                    index += skip;
                    continue;
                } else {
                    index += 1;
                    continue;
                }
            }
        };
    }
    tokens
}

fn string(
    line: String,
    line_count: usize,
    start_param: usize,
) -> Result<(Token, usize), ScanError> {
    let start = start_param + 1;
    let mut current_index = start;
    loop {
        let char = line_range(line.clone(), current_index, current_index + 1);
        if char == "\n" || char == "\0" || char == "\"" {
            break;
        } else {
            current_index += 1;
            continue;
        }
    }
    let char = line.get(current_index..current_index + 1).unwrap();
    if char == "\"" {
        let literal_str = invert_escape(line.get(start..current_index).unwrap().to_string());
        let literal = Literal::String(literal_str);
        Ok((
            Token {
                token_type: TokenType::String,
                line: line_count,
                col: start,
                literal: Some(literal),
            },
            current_index + 2 - start,
        ))
    } else if char == "\n" || char == "\0" {
        Err(ScanError {
            message: "Unterminated string".to_string(),
            col: start,
        })
    } else {
        Err(ScanError {
            message: format!("Expected a `\"` instead `{}`", char),
            col: start,
        })
    }
}

fn identifier(line: String, line_count: usize, start_param: usize) -> (Token, usize) {
    let start = start_param;
    let mut current_index = start;
    loop {
        if current_index >= line.len() {
            break;
        }
        let char = line_range(line.clone(), current_index, current_index + 1);
        if is_alphanumeric(char.as_str()) {
            current_index += 1;
            continue;
        } else {
            break;
        }
    }
    let text = line.get(start..current_index).unwrap();
    let binding = get_keywords_hash();
    let token_type = binding.get(text);
    match token_type {
        Some(type_value) => (
            Token {
                token_type: type_value.clone(),
                line: line_count,
                col: start,
                literal: None,
            },
            current_index - start,
        ),
        None => (
            Token {
                token_type: TokenType::Identifier,
                literal: Some(Literal::Identifier(text.to_string())),
                line: line_count,
                col: start,
            },
            current_index - start,
        ),
    }
}

fn number(
    line: String,
    line_count: usize,
    start_param: usize,
) -> Result<(Token, usize), ScanError> {
    let start = start_param;
    let mut current_index = start;
    loop {
        if current_index >= line.len() {
            break;
        }
        let char = line_range(line.clone(), current_index, current_index + 1);
        if is_numeric(char.as_str()) {
            current_index += 1;
            continue;
        } else {
            break;
        }
    }

    let char = line_range(line.clone(), current_index - 1, current_index);
    let prev_char = line_range(line.clone(), current_index - 2, current_index - 1);
    let next_char = match current_index < line.len() {
        true => line_range(line.clone(), current_index + 1, current_index + 2),
        false => '\0'.to_string(),
    };
    let incorrect_float_number = is_numeric(prev_char.as_str())
        && char == "."
        && (next_char == "\0" || is_alpha(next_char.as_str()));

    if incorrect_float_number || is_alpha(char.as_str()) {
        Err(ScanError {
            message: format!("Expected a `number` instead `{}`", next_char),
            col: current_index + 2,
        })
    } else {
        let line = line.get(start..current_index).unwrap();
        let the_number: f64 = line.parse().unwrap();
        let literal = Literal::Number(the_number);
        Ok((
            Token {
                token_type: TokenType::Number,
                line: line_count,
                col: start,
                literal: Some(literal),
            },
            current_index - start,
        ))
    }
}

fn line_range(line: String, start: usize, end: usize) -> String {
    let line_as_vec_chars = line.chars().collect::<Vec<_>>();
    let line_string_slice = line_as_vec_chars[start..end]
        .iter()
        .cloned()
        .collect::<String>();
    line_string_slice
}

#[cfg(test)]
mod tests {
    use crate::spec::{
        scanner::scan_tokens,
        types::{Literal, TokenType},
    };

    #[test]
    fn should_identify_a_print() {
        let input = r#"
            print("Hello World")
        "#;
        let result = scan_tokens(input.to_string());
        let tokens = result.tokens;
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Print);
        assert_eq!(tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(tokens[2].token_type, TokenType::String);
        assert_eq!(tokens[3].token_type, TokenType::RightParen);
    }

    #[test]
    fn should_identify_a_string() {
        let input = r#"
            "Hello World"
        "#;
        let result = scan_tokens(input.to_string());
        let tokens = result.tokens;
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(
            tokens[0].literal,
            Some(Literal::String("Hello World".to_string()))
        );
    }

    #[test]
    fn should_identify_a_literal_number() {
        let input = r#"
            10
        "#;
        let result = scan_tokens(input.to_string());
        let tokens = result.tokens;
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].literal, Some(Literal::Number(10_f64)));
    }

    #[test]
    fn should_identify_keywords() {
        let input = r#"
            severo
            print
        "#;
        let result = scan_tokens(input.to_string());
        let tokens = result.tokens;
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Print);
    }
}
