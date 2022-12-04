pub fn convert(s: String, num_rows: i32) -> String {
    let mut char_grid: Vec<Vec<char>> = vec![];
    let mut result_string: String = "".to_string();

    for _ in 0..num_rows {
        char_grid.push(vec![]);
    }

    let iter1 = 0..num_rows;
    let iter2 = 1..num_rows - 1;
    let mut vector: Vec<i32> = iter1.chain(iter2.rev()).collect();

    while vector.len() < s.len() {
        vector.append(&mut vector.clone()); //TODO: avoid cloning
    }

    for (index, char) in s.chars().enumerate() {
        char_grid[vector[index as usize] as usize].push(char);
    }

    for i in 0..num_rows {
        let new_string: String = char_grid[i as usize].iter().collect();
        result_string += &new_string[..];
    }

    result_string
}
