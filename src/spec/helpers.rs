pub fn rest_matches(start: usize, line: &str, next_chars: &str) -> bool {
    let rest_len = next_chars.len();
    match line.get(start..(start + rest_len)) {
        Some(str_part) => next_chars == str_part,
        None => false,
    }
}

pub fn is_numeric(str: &str) -> bool {
    str.chars().all(|c| c.is_ascii_digit())
}

pub fn is_alpha(str: &str) -> bool {
    str.chars().all(|c| c.is_ascii_alphabetic())
}

pub fn is_alphanumeric(str: &str) -> bool {
    str.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn invert_escape(src: String) -> String {
    src.replace("\\b", "\x08")
        .replace("\\f", "\x0c")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rest_matches() {
        let line = "Hello, world!";
        assert!(rest_matches(0, line, "Hello"));
    }

    #[test]
    fn test_rest_not_matches() {
        let line = "Hello, world!";
        assert!(!rest_matches(0, line, "Hellow"));
    }

    #[test]
    fn test_rest_not_has_valid_index() {
        let line = "Hello, world!";
        assert!(!rest_matches(line.len(), line, "!"));
    }
}
