pub fn roman_to_int(s: String) -> i32 {
    let mut result: i32 = 0;
    let mut last_character: char = ' ';
    for character in s.chars() {
        match character {
            'I' => {
                result += 1;
            }
            'V' => {
                if last_character == 'I' {
                    result += 3;
                } else {
                    result += 5;
                }
            }
            'X' => {
                if last_character == 'I' {
                    result += 8;
                } else {
                    result += 10;
                }
            }
            'L' => {
                if last_character == 'X' {
                    result += 30;
                } else {
                    result += 50;
                }
            }
            'C' => {
                if last_character == 'X' {
                    result += 80;
                } else {
                    result += 100;
                }
            }
            'D' => {
                if last_character == 'C' {
                    result += 300;
                } else {
                    result += 500;
                }
            }
            'M' => {
                if last_character == 'C' {
                    result += 800;
                } else {
                    result += 1000;
                }
            }
            _ => {}
        }
        last_character = character;
    }

    result
}
