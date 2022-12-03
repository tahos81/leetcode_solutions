pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    nums1.append(&mut nums2);
    nums1.sort();
    let length = nums1.len();

    if length % 2 == 0 {
        ((nums1[length / 2] as f64) + (nums1[length / 2 - 1] as f64)) / 2.0
    } else {
        nums1[length / 2] as f64
    }
}
