use std::cmp::max;
use std::cmp::min;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut current_max: i32 = 0;
    let left_candidates = height
        .iter()
        .enumerate()
        .filter(|(_, val)| {
            let predicate: bool = val > &&current_max;
            current_max = max(current_max, **val);
            predicate
        })
        .collect::<Vec<(usize, &i32)>>();

    current_max = 0;
    let right_candidates = height
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, val)| {
            let predicate: bool = val > &&current_max;
            current_max = max(current_max, **val);
            predicate
        })
        .collect::<Vec<(usize, &i32)>>();

    let mut max_area = 0;
    let length = height.len();

    for (i_r, val_r) in right_candidates {
        for (i_l, val_l) in &left_candidates {
            if i_l >= &(length - 1 - i_r) {
                break;
            }

            max_area = max(
                max_area,
                ((length - 1 - i_r) - i_l) as i32 * min(val_r, val_l),
            )
        }
    }

    max_area
}
