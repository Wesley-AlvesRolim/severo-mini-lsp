pub fn get_word_in_line_col_position(
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
        if current_char.is_whitespace() || current_char == '(' {
            start_word = col_index + 1;
            break;
        } else {
            start_word = col_index;
        }
    }

    for col_index in cursor_column_position..line_len {
        let current_char = line.chars().nth(col_index).unwrap();
        if current_char.is_whitespace() || current_char == '(' || current_char == ')' {
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
        let (found_word, start, end) = get_word_in_line_col_position(line.clone(), line.len() + 1);
        let expected_word = "".to_string();
        assert_eq!(expected_word, found_word);
        assert_eq!(start, 0);
        assert_eq!(end, 0);
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

    #[test]
    fn should_found_word_when_is_a_builtin_function() {
        let line = "print(\"Hello World\")".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line, 3);
        let expected_word = "print".to_string();
        assert_eq!(expected_word, found_word);
    }

    #[test]
    fn should_found_word_when_is_inside_call_of_function() {
        let line = "print(var)".to_string();
        let (found_word, _, _) = get_word_in_line_col_position(line, 7);
        let expected_word = "var".to_string();
        assert_eq!(expected_word, found_word);
    }

    #[test]
    fn test_no_word_found() {
        let line = "   ";
        let (found_word, start, end) = get_word_in_line_col_position(line.to_string(), 0);
        let expected_word = "".to_string();
        assert_eq!(expected_word, found_word);
        assert_eq!(start, 0);
        assert_eq!(end, 0);
    }
}
