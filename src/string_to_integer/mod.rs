use std::i32::{MAX, MIN};
use std::num::ParseIntError;

pub fn my_atoi(s: String) -> i32 {
    let trimmed_s: &str = s.trim_start();
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
            std::num::IntErrorKind::PosOverflow => MAX,
            std::num::IntErrorKind::NegOverflow => MIN,
            _ => 0,
        })
}
