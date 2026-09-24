mod integers;

// Problem:
// Given a list of integers, use a vector and return the median (when sorted, the value in the 
// middle position) and mode (the value that occurs most often; a hash map will be helpful here) 
// of the list.
fn main() {
    assert_eq!(integers::median(&[7, 1, 5, 9, 2]), Some(5));
    assert_eq!(integers::median(&[15, 4, 9, 18, 51, 3]), Some(12));

    assert_eq!(integers::median(&[1, 1, 1, 2, 2, 3, 3, 3, 3]), Some(2));
    assert_eq!(integers::mode(&[1, 1, 1, 2, 2, 3, 3, 3, 3]), Some(3));

    assert_eq!(integers::median(&[]), None);
    assert_eq!(integers::mode(&[]), None);
}
