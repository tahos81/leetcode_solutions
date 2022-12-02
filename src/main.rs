use crate::add_two_numbers::add_two_numbers;
use crate::add_two_numbers::ListNode;

mod add_two_numbers;
mod two_sum;

fn main() {
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

    println!("{:?}", add_two_numbers(num1, num2));
}
