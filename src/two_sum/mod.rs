use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result_vector = Vec::new();
    let mut memoization: HashMap<i32, usize> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let value = memoization.get(num);
        match value {
            Some(value) => {
                result_vector.push(*value as i32);
                result_vector.push(index as i32);
            }
            None => {
                memoization.insert(target - num, index);
            }
        }
    }
    result_vector
}
