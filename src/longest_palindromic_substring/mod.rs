pub fn longest_palindrome(s: String) -> String {
    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() {
        let len1 = expand_around_center(&s, i, i);
        let len2 = expand_around_center(&s, i, i + 1);
        let len = std::cmp::max(len1, len2);

        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }

    s[start..end + 1].to_string()
}

pub fn expand_around_center(s: &str, mut left: usize, mut right: usize) -> usize {
    while left != (2_usize.pow(64) - 1) //TODO: prevent panic at development mode
        && right < s.len()
        && s.as_bytes()[left] == s.as_bytes()[right]
    {
        left -= 1;
        right += 1;
    }

    return right - left - 1;
}
