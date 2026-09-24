use std::collections::HashMap;
// Problem:
// Given a list of integers, use a vector and return the median (when sorted, the value in the 
// middle position) and mode (the value that occurs most often; a hash map will be helpful here) 
// of the list.

/// Returns the median of `numbers`, or `None` if the list is empty.
///
/// For an even number of elements, the median is the average of the two
/// middle values (rounded towards zero, since we work with integers).
pub fn median(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut sorted = numbers.to_vec();
    sorted.sort();

    let middle = sorted.len() / 2;
    let median = if sorted.len() % 2 == 1 {
        sorted[middle]
    } else {
        (sorted[middle - 1] + sorted[middle]) / 2
    };

    Some(median)
}

/// Returns the value that appears most often in `numbers`,
/// or `None` if the list is empty.
pub fn mode(numbers: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for number in numbers {
        *counts.entry(*number).or_insert(0) += 1;
    }

    let mut mode = None;
    let mut highest_count = 0;
    for (number, count) in &counts {
        if *count > highest_count {
            highest_count = *count;
            mode = Some(*number);
        }
    }

    mode
}
