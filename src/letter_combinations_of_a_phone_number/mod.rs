pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if digits.len() == 0 {
        return result;
    }
    result.push("".to_string());

    let telephone: Vec<&str> = Vec::from([
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ]);

    for digit in digits
        .chars()
        .map(|char| char.to_string().parse::<usize>().unwrap())
    {
        let mut new_result: Vec<String> = Vec::new();
        for word in result.iter() {
            for letter in telephone[digit].chars() {
                new_result.push(format!("{}{}", word, letter));
            }
        }
        result = new_result;
    }

    result
}
