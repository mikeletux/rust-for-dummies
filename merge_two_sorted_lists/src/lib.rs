//! # 21. Merge Two Sorted Lists
//!
//! You are given the heads of two sorted linked lists, `list1` and `list2`.
//! Merge them into one sorted list by splicing together the nodes of the two
//! lists. Return the head of the merged linked list.
//!
//! ## Examples
//!
//! - `list1 = [1, 2, 4]`, `list2 = [1, 3, 4]` returns `[1, 1, 2, 3, 4, 4]`.
//! - `list1 = []`, `list2 = []` returns `[]`.
//! - `list1 = []`, `list2 = [0]` returns `[0]`.
//!
//! ## Constraints
//!
//! - The total number of nodes in both lists is between 0 and 50.
//! - `-100 <= Node.val <= 100`
//! - Both lists are sorted in non-decreasing order.
//!
//! Source: <https://leetcode.com/problems/merge-two-sorted-lists/>

/// A node in a singly linked list, matching LeetCode's Rust definition.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

/// Merges two sorted linked lists into a single sorted linked list.
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0); // We do not do anything with this
    let mut tail = &mut dummy;
    let mut head1 = list1;
    let mut head2 = list2;


    while let (Some(node1), Some(node2)) = (head1.as_ref(), head2.as_ref()) {
        if node1.val <= node2.val {
            let mut node = head1.take().unwrap();
            head1 = node.next.take();
            tail.next = Some(node);
        } else {
            let mut node = head2.take().unwrap();
            head2 = node.next.take();
            tail.next = Some(node);
        }
        tail = tail.next.as_mut().unwrap();
    }
    // Attach the remaning nodes
    if head1 == None {
        tail.next = head2;
    } else {
        tail.next = head1;
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a linked list whose nodes follow the order of the given values.
    fn list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn example_1_merges_two_non_empty_lists() {
        assert_eq!(
            merge_two_lists(list(&[1, 2, 4]), list(&[1, 3, 4])),
            list(&[1, 1, 2, 3, 4, 4]),
        );
    }

    #[test]
    fn example_2_merges_two_empty_lists() {
        assert_eq!(merge_two_lists(None, None), None);
    }

    #[test]
    fn example_3_merges_empty_list_with_zero() {
        assert_eq!(merge_two_lists(None, list(&[0])), list(&[0]));
    }

    #[test]
    fn returns_first_list_when_second_is_empty() {
        assert_eq!(merge_two_lists(list(&[1, 2, 3]), None), list(&[1, 2, 3]));
    }

    #[test]
    fn merges_negative_values_and_lists_of_different_lengths() {
        assert_eq!(
            merge_two_lists(list(&[-100, -3, 0, 100]), list(&[-5, 2])),
            list(&[-100, -5, -3, 0, 2, 100]),
        );
    }

    #[test]
    fn preserves_duplicate_values() {
        assert_eq!(
            merge_two_lists(list(&[1, 1]), list(&[1, 1, 1])),
            list(&[1, 1, 1, 1, 1]),
        );
    }
}
