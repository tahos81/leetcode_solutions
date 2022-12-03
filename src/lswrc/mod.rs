use std::cmp::max;
use std::collections::HashSet;
//"dvdf"
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest: usize = 0;
    let mut set: HashSet<char> = HashSet::new();
    for index in 0..s.len() {
        let s_slice = &s[index..];
        for character in s_slice.chars() {
            if set.contains(&character) {
                break;
            }
            set.insert(character);
        }
        longest = max(longest, set.len());
        set.clear();
    }

    longest as i32
}
