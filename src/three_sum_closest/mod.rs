pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort();
    let mut min_diff: i32 = std::i32::MAX;

    for index in 0..nums.len() {
        let new_diff = two_sum_closest(&nums, target - nums[index], index);
        if new_diff.abs() < min_diff.abs() {
            min_diff = new_diff;
        }
    }

    target + min_diff
}

pub fn two_sum_closest(nums: &Vec<i32>, target: i32, skip_index: usize) -> i32 {
    let mut i: usize = 0;
    let mut j: usize = nums.len() - 1;
    let mut target_difference = std::i32::MAX;

    while i < j {
        if skip_index == i {
            i += 1;
            continue;
        } else if skip_index == j {
            j -= 1;
            continue;
        }

        let current_number: i32 = nums[i] + nums[j] - target;

        if current_number.abs() < target_difference.abs() {
            target_difference = current_number;
        }

        if target_difference > 0 {
            j -= 1;
        } else if target_difference < 0 {
            i += 1;
        } else {
            return 0_i32;
        }
    }

    target_difference
}
