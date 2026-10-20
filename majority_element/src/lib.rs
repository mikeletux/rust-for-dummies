//! # 169. Majority Element
//!
//! Given an array `nums` of size `n`, return its majority element.
//!
//! The majority element appears more than `floor(n / 2)` times. You may assume
//! that a majority element always exists in the array.
//!
//! ## Examples
//!
//! - `[3, 2, 3]` returns `3`.
//! - `[2, 2, 1, 1, 1, 2, 2]` returns `2`.
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 50_000`
//! - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
//! - A majority element always exists in the array.
//!
//! ## Follow-up
//!
//! Can you solve the problem in linear time and with constant extra space?
//!
//! Source: <https://leetcode.com/problems/majority-element/>

/// Returns the element that appears more than half the time in `nums`.
/// 
use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let threshold = nums.len() / 2;
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for num in nums {
        let count = counts
            .entry(num)
            .or_insert(0);

        *count += 1;
        if *count > threshold as i32 {
            return num
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_majority_element_is_three() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_2_majority_element_is_two() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
