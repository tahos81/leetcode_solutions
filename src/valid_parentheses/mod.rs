pub fn is_valid(s: String) -> bool {
    let mut char_vec: Vec<char> = Vec::new();

    for character in s.chars() {
        if character == '(' {
            char_vec.push(')');
        } else if character == '[' {
            char_vec.push(']');
        } else if character == '{' {
            char_vec.push('}');
        } else if character != *char_vec.get(char_vec.len() - 1).unwrap_or(&' ') {
            return false;
        } else {
            char_vec.pop();
        }
    }

    char_vec.len() == 0
}
