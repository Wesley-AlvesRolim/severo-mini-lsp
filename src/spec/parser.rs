use super::types::{Literal, Token, TokenType};

pub fn get_vars(tokens: Vec<Token>) -> Vec<String> {
    let mut vars = Vec::new();
    let mut index = 0;

    while index < tokens.len() {
        let token = tokens.get(index).unwrap();
        if token.token_type == TokenType::Var {
            let next_token = tokens.get(index + 1);
            if let Some(next_token) = next_token {
                let un_next_token_type = &next_token.token_type;
                let un_next_token_literal = &next_token.literal;
                if un_next_token_type == &TokenType::Identifier {
                    let v = un_next_token_literal;
                    if let Some(Literal::Identifier(var_name)) = v {
                        vars.push(var_name.clone());
                    }
                }
            }
        }
        index += 1;
    }

    vars
}

#[cfg(test)]
mod tests {
    use crate::spec::types::{Literal, Token, TokenType};

    use super::*;

    #[test]
    fn should_found_a_variable() {
        let variable_name = "variableName".to_string();
        let tokens = vec![
            Token {
                token_type: TokenType::Var,
                line: 0,
                col: 0,
                literal: None,
            },
            Token {
                token_type: TokenType::Identifier,
                line: 0,
                col: 0,
                literal: Some(Literal::Identifier(variable_name.clone())),
            },
        ];
        let found_vars = get_vars(tokens);
        let expected_vars = vec![variable_name];
        assert_eq!(expected_vars, found_vars);
    }

    #[test]
    fn should_return_empty_when_a_variable_is_incomplete() {
        let tokens = vec![Token {
            token_type: TokenType::Var,
            line: 0,
            col: 0,
            literal: None,
        }];
        let found_vars = get_vars(tokens);
        let expected_vars: Vec<String> = vec![];
        assert_eq!(expected_vars, found_vars);
    }

    #[test]
    fn should_return_empty_when_not_has_a_variable() {
        let tokens = vec![
            Token {
                token_type: TokenType::Print,
                line: 0,
                col: 0,
                literal: None,
            },
            Token {
                token_type: TokenType::LeftParen,
                line: 0,
                col: 0,
                literal: None,
            },
            Token {
                token_type: TokenType::Var,
                line: 0,
                col: 0,
                literal: None,
            },
            Token {
                token_type: TokenType::RightParen,
                line: 0,
                col: 0,
                literal: None,
            },
        ];
        let found_vars = get_vars(tokens);
        let expected_vars: Vec<String> = vec![];
        assert_eq!(expected_vars, found_vars);
    }
}
