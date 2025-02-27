use std::collections::HashMap;

use super::types::TokenType;

pub fn get_keywords() -> Vec<String> {
    vec!["severo".to_string()]
}

pub fn get_keywords_hash() -> HashMap<&'static str, TokenType> {
    let mut keywords_map = HashMap::new();
    keywords_map.insert("severo", TokenType::Var);
    keywords_map.insert("print", TokenType::Print);
    keywords_map
}
