//! # 697. Degree of an Array
//!
//! Given a non-empty array of non-negative integers `nums`, its **degree** is
//! the maximum frequency of any one of its elements.
//!
//! Find the smallest possible length of a contiguous subarray of `nums` that
//! has the same degree as `nums`.
//!
//! ## Examples
//!
//! - `[1, 2, 2, 3, 1]` returns `2`. The array has degree 2, and `[2, 2]` is
//!   the shortest subarray with that degree.
//! - `[1, 2, 2, 3, 1, 4, 2]` returns `6`. The array has degree 3, and
//!   `[2, 2, 3, 1, 4, 2]` is the shortest subarray with that degree.
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 50_000`
//! - `0 <= nums[i] <= 49_999`
//!
//! Source: <https://leetcode.com/problems/degree-of-an-array/>
/// Returns the length of the shortest contiguous subarray with the same degree
/// as `nums`.
/// 

use std::collections::HashMap;

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut list: HashMap<i32, Vec<i32>> = HashMap::new();

    for (i, v) in nums.iter().enumerate() {
        list
            .entry(*v)
            .and_modify(|e| e.push(i as i32))
            .or_insert(vec![i as i32]);
    }

    // Get most repeated value
    if let Some(highest_degree_value) = list
        .iter()
        .max_by(|x, y| x.1.len().cmp(&y.1.len()).then_with(|| {
            let x_distance = x.1.iter().max().unwrap() - x.1.iter().min().unwrap();
            let y_distance = y.1.iter().max().unwrap() - y.1.iter().min().unwrap();

            y_distance.cmp(&x_distance)
        })) 
        {
            // Get min and max indexes
            let min = highest_degree_value.1.iter().min().unwrap();
            let max = highest_degree_value.1.iter().max().unwrap();

            return max - min + 1;
        };

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_shortest_subarray_has_length_two() {
        assert_eq!(find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    }

    #[test]
    fn example_2_shortest_subarray_has_length_six() {
        assert_eq!(find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]), 6);
    }
}
