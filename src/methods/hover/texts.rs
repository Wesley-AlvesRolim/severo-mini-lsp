pub fn get_hover_text(word: String) -> Option<String> {
    match word.as_str() {
        "severo" => Some(VAR.to_string()),
        "print" => Some(PRINT.to_string()),
        _ => None,
    }
}

pub const VAR: &str = r#"
### severo

It's a keyword to declare a variable. Here are some examples:
```severo
severo hello = "Hello World"
severo num = 10
```"#;

pub const PRINT: &str = r#"
### print

It's print something. Here are some examples:
```severo
severo hello = "Hello World"
print(hello)
```"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hover_text_for_severo() {
        let word = "severo".to_string();
        let expected = Some(VAR.to_string());
        assert_eq!(get_hover_text(word), expected);
    }

    #[test]
    fn test_get_hover_text_for_print() {
        let word = "print".to_string();
        let expected = Some(PRINT.to_string());
        assert_eq!(get_hover_text(word), expected);
    }

    #[test]
    fn test_get_hover_text_for_unknown_keyword() {
        let word = "unknown".to_string();
        let expected = None;
        assert_eq!(get_hover_text(word), expected);
    }

    #[test]
    fn test_get_hover_text_empty_string() {
        let word = "".to_string();
        let expected = None;
        assert_eq!(get_hover_text(word), expected);
    }

    #[test]
    fn test_get_hover_text_case_sensitivity() {
        let word = "SeVeRo".to_string();
        let expected = None;
        assert_eq!(get_hover_text(word), expected);
    }

    #[test]
    fn test_get_hover_text_special_characters() {
        let word = "severo!".to_string();
        let expected = None;
        assert_eq!(get_hover_text(word), expected);
    }
}
