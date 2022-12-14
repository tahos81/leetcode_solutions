pub fn is_match(s: String, p: String) -> bool {
    regex(s.as_str(), p.as_str())
}

fn regex(s: &str, p: &str) -> bool {
    if p == "" {
        return s == "";
    }

    let first_match = s != ""
        && (p.as_bytes().get(0) == s.as_bytes().get(0) || p.chars().nth(0).unwrap_or(' ') == '.');

    if p.len() >= 2 && p.chars().nth(1) == Some('*') {
        regex(&s[..], &p[2..]) || first_match && regex(&s[1..], &p[..])
    } else {
        first_match && regex(&s[1..], &p[1..])
    }
}
