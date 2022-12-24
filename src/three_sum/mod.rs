use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut skip_numbers: HashSet<i32> = HashSet::new();
    let mut skip_index: usize = 0;

    for num in nums.iter() {
        if skip_numbers.contains(num) {
            continue;
        }
        two_sum(&nums, -num, &mut skip_numbers, &mut result, skip_index);
        skip_numbers.insert(*num);
        skip_index += 1;
    }

    result
}

pub fn two_sum(
    nums: &Vec<i32>,
    target: i32,
    skip_numbers: &mut HashSet<i32>,
    result: &mut Vec<Vec<i32>>,
    skip_index: usize,
) {
    let mut memoization: HashSet<i32> = HashSet::new();
    for index in 0..nums.len() {
        if skip_numbers.contains(&nums[index]) || index == skip_index {
            continue;
        }
        let value = memoization.get(&nums[index]);

        match value {
            Some(value) => {
                if !result.contains(&Vec::from([-target, *value, target - value])) {
                    result.push(Vec::from([-target, *value, target - value]));
                }
            }
            None => {
                memoization.insert(target - nums[index]);
            }
        };
    }
}
