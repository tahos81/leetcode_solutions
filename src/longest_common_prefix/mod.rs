pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result: String = String::from("");
    'outer: for index in 0..strs[0].len() {
        let cmp_str = &strs[0].get(index..index + 1).unwrap_or("");
        for word in strs.iter().skip(1) {
            if cmp_str != &word.get(index..index + 1).unwrap_or("") {
                break 'outer;
            }
        }
        result.push_str(cmp_str);
    }

    result
}
