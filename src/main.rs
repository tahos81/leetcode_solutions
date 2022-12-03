use crate::add_two_numbers::add_two_numbers;
use crate::add_two_numbers::ListNode;
use crate::lswrc::length_of_longest_substring;
use crate::two_sum::two_sum;

mod add_two_numbers;
mod lswrc;
mod two_sum;

fn main() {
    test_two_sum();
    test_add_two_number();
    test_lswrc()
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
