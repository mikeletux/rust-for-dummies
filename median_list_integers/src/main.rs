mod integers;

// Problem:
// Given a list of integers, use a vector and return the median (when sorted, the value in the 
// middle position) and mode (the value that occurs most often; a hash map will be helpful here) 
// of the list.
fn main() {
    // let mut vec1 = vec![7, 1, 5, 9, 2];
    // let mut vec2 = vec![15, 4, 9, 18, 51, 3];
    let mut vec3 = vec![1, 1, 1, 2, 2, 3, 3, 3, 3];

    // assert_eq!(integers::get_median_and_mode(&mut vec1), 5);
    // assert_eq!(integers::get_median_and_mode(&mut vec2), 12);
    assert_eq!(integers::get_median_and_mode(&mut vec3), (2, 3));
}
