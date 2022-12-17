pub fn int_to_roman(num: i32) -> String {
    let mut result: String = String::from("");

    num.to_string()
        .as_bytes()
        .iter()
        .rev()
        .enumerate()
        .rev()
        .map(|(index, byte)| {
            let mut digit = (*byte as char).to_string().parse::<usize>().unwrap();
            match index {
                0 => {
                    if digit < 4 {
                        result.push_str(&"I".repeat(digit))
                    } else if digit == 4 {
                        result.push_str(&"IV")
                    } else if digit == 9 {
                        result.push_str(&"IX")
                    } else {
                        result.push_str(&"V");
                        digit -= 5;
                        result.push_str(&"I".repeat(digit));
                    }
                }
                1 => {
                    if digit < 4 {
                        result.push_str(&"X".repeat(digit))
                    } else if digit == 4 {
                        result.push_str(&"XL")
                    } else if digit == 9 {
                        result.push_str(&"XC")
                    } else {
                        result.push_str(&"L");
                        digit -= 5;
                        result.push_str(&"X".repeat(digit));
                    }
                }
                2 => {
                    if digit < 4 {
                        result.push_str(&"C".repeat(digit))
                    } else if digit == 4 {
                        result.push_str(&"CD")
                    } else if digit == 9 {
                        result.push_str(&"CM")
                    } else {
                        result.push_str(&"D");
                        digit -= 5;
                        result.push_str(&"C".repeat(digit));
                    }
                }
                _ => result.push_str(&"M".repeat(10_usize.pow(index as u32 - 3) * digit)),
            }
        })
        .count();

    result
}
