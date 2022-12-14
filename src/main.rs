//use crate::add_two_numbers::add_two_numbers;
use crate::add_two_numbers::ListNode;
use crate::longest_palindromic_substring::longest_palindrome;
use crate::longest_substring_without_repeating_characters::length_of_longest_substring;
use crate::palindrome_number::is_palindrome;
use crate::regular_expression_matching::is_match;
use crate::reverse_integer::reverse;
use crate::string_to_integer::my_atoi;
use crate::two_sum::two_sum;
use crate::zigzag_conversion::convert;

mod add_two_numbers;
mod longest_palindromic_substring;
mod longest_substring_without_repeating_characters;
mod median_of_two_sorted_arrays;
mod palindrome_number;
mod regular_expression_matching;
mod reverse_integer;
mod string_to_integer;
mod two_sum;
mod zigzag_conversion;

fn main() {
    test_two_sum();
    test_add_two_number();
    test_longest_substring_without_repeating_characters();
    test_longest_palindromic_substring();
    test_zigzag_conversion();
    test_reverse_integer();
    test_string_to_integer();
    test_palindrome_number();
    test_regular_expression_matching();
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

fn test_longest_substring_without_repeating_characters() {
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

fn test_reverse_integer() {
    let test_int: i32 = 123;
    assert_eq!(321, reverse(test_int));
}

fn test_string_to_integer() {
    let test_string: String = String::from("   123a123");
    assert_eq!(123, my_atoi(test_string));
}

fn test_palindrome_number() {
    let test_number: i32 = 353;
    assert!(is_palindrome(test_number));
}

fn test_regular_expression_matching() {
    let test_s: String = String::from("tahhhh");
    let test_p: String = String::from("tac*h*");
    assert!(is_match(test_s, test_p));
}
