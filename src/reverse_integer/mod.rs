pub fn reverse(x: i32) -> i32 {
    let mut reversed_string = x.to_string().chars().rev().collect::<String>();
    if x >= 0 {
        reversed_string.parse::<i32>().unwrap_or(0)
    } else {
        reversed_string = reversed_string[..reversed_string.len() - 1].to_string();
        (-1) * reversed_string.parse::<i32>().unwrap_or(0)
    }
}
