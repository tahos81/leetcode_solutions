//use crate::add_two_numbers::add_two_numbers;
use crate::add_two_numbers::ListNode;
use crate::longest_palindromic_substring::longest_palindrome;
use crate::lswrc::length_of_longest_substring;
use crate::two_sum::two_sum;
use crate::zigzag_conversion::convert;

mod add_two_numbers;
mod longest_palindromic_substring;
mod lswrc;
mod median_of_two_sorted_arrays;
mod two_sum;
mod zigzag_conversion;

fn main() {
    test_two_sum();
    test_add_two_number();
    test_lswrc();
    test_longest_palindromic_substring();
    test_zigzag_conversion();
}

fn test_two_sum() {
    let test_vec = vec![5, 2, 7, 8, 15];
    let test_target = 10;
    let result = two_sum(test_vec, test_target);
    assert!(result == [1, 3] || result == [3, 1]);
}

fn test_add_two_number() {
    let num1 = Some(Box::new(ListNode { val: 9, next: None }));

    let num2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode {
                                        val: 9,
                                        next: Some(Box::new(ListNode { val: 9, next: None })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        })),
    }));
}

fn test_lswrc() {
    let test_string = "pwwkew".to_string();
    let length = length_of_longest_substring(test_string);
    assert_eq!(length, 3);
}

fn test_longest_palindromic_substring() {
    let test_string = "babad".to_string();
    let expected_result = "aba".to_string();
    assert_eq!(expected_result, longest_palindrome(test_string));
}

fn test_zigzag_conversion() {
    let test_string: String = "PAYPALISHIRING".to_string();
    let test_num_rows: i32 = 4;

    assert_eq!(
        "PINALSIGYAHRPI".to_string(),
        convert(test_string, test_num_rows)
    );
}
