#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry: i32 = 0;
    let mut digit: i32;

    let mut result_list: Option<Box<ListNode>> = None;
    let mut current_node: &mut Option<Box<ListNode>> = &mut result_list;
    let mut initialized = false;

    loop {
        match &l1 {
            Some(l1_box_node) => match &l2 {
                Some(l2_box_node) => {
                    digit = l1_box_node.val + l2_box_node.val + carry;
                    carry = digit / 10;
                    digit = digit % 10;
                    l1 = l1_box_node.next.clone();
                    l2 = l2_box_node.next.clone();
                    let new_node = Some(Box::new(ListNode::new(digit)));

                    if initialized {
                        current_node.as_mut().unwrap().next = new_node;
                        current_node = &mut current_node.as_mut().unwrap().next;
                    } else {
                        *current_node = new_node;
                        initialized = true;
                    }
                }
                None => {
                    digit = l1_box_node.val + carry;
                    carry = digit / 10;
                    digit = digit % 10;
                    l1 = l1_box_node.next.clone();
                    let new_node = Some(Box::new(ListNode::new(digit)));

                    if initialized {
                        current_node.as_mut().unwrap().next = new_node;
                        current_node = &mut current_node.as_mut().unwrap().next;
                    } else {
                        *current_node = new_node;
                        initialized = true;
                    }
                }
            },
            None => match &l2 {
                Some(l2_box_node) => {
                    digit = l2_box_node.val + carry;
                    carry = digit / 10;
                    digit = digit % 10;
                    l2 = l2_box_node.next.clone();
                    let new_node = Some(Box::new(ListNode::new(digit)));

                    if initialized {
                        current_node.as_mut().unwrap().next = new_node;
                        current_node = &mut current_node.as_mut().unwrap().next;
                    } else {
                        *current_node = new_node;
                        initialized = true;
                    }
                }
                None => {
                    if carry != 0 {
                        let new_node = Some(Box::new(ListNode::new(carry)));
                        current_node.as_mut().unwrap().next = new_node;
                        // current_node = &mut current_node.as_mut().unwrap().next;
                    }
                    break;
                }
            },
        }
    }

    result_list
}
