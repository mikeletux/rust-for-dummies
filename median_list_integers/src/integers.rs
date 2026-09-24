use std::collections::HashMap;
// Problem:
// Given a list of integers, use a vector and return the median (when sorted, the value in the 
// middle position) and mode (the value that occurs most often; a hash map will be helpful here) 
// of the list.

pub fn get_median_and_mode(list_of_integers: &mut Vec<i32>) -> (i32, i32) {
    let median: i32;
    let mut occurances: HashMap<i32, i32> = HashMap::new();

    list_of_integers.sort();

    if list_of_integers.len() % 2 == 1 {
        median = list_of_integers[list_of_integers.len() / 2]
    } else {
        median = (list_of_integers[list_of_integers.len() / 2] + (list_of_integers[(list_of_integers.len() / 2) - 1] )) / 2;
    }

    for value in list_of_integers {
        *occurances.entry(*value).or_insert(0) += 1;
    }
    
    // We haven't seen (yet!) iterators or closures so:

    let mut mode = 0;
    let mut mode_value = i32::MIN;
    for (k, v) in &occurances {
        if *v > mode_value {
            mode_value = *v;
            mode = *k;
        }
    }

    (median, mode)
}