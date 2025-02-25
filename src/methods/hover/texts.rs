pub fn get_hover_text(word: String) -> Option<String> {
    match word.as_str() {
        "severo" => Some(VAR.to_string()),
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
