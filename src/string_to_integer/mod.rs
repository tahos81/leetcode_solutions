use std::i32;
use std::num::IntErrorKind;
use std::num::ParseIntError;

pub fn my_atoi(s: String) -> i32 {
    let trimmed_s: &str = s.trim();
    let string_to_parse: Option<&str> = trimmed_s
        .find(|character: char| !character.is_numeric())
        .and_then(|index: usize| match index {
            //okey to use unwrap() since it can not panic
            0 => match trimmed_s.chars().nth(0).unwrap() {
                '-' | '+' => trimmed_s[1..]
                    .find(|character: char| !character.is_numeric())
                    .and_then(|index: usize| Some(&trimmed_s[..index + 1])),
                _ => None,
            },
            _ => Some(&trimmed_s[..index]),
        });

    string_to_parse
        .unwrap_or(trimmed_s)
        .parse::<i32>()
        .unwrap_or_else(|err: ParseIntError| match err.kind() {
            IntErrorKind::PosOverflow => i32::MAX,
            IntErrorKind::NegOverflow => i32::MIN,
            _ => 0,
        })
}
