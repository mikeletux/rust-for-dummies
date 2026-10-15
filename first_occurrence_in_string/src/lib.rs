/// Returns the index of the first occurrence of `needle` in `haystack`, or -1
/// when `needle` does not occur in `haystack`.
pub fn str_str(haystack: String, needle: String) -> i32 {
    // Some sanity checks
    if !haystack.contains(&needle) || needle.len() > haystack.len() || needle.is_empty() || haystack.is_empty() {
        return -1;
    }
    
    // haystack
    //     .match_indices(&needle)
    //     .next()
    //     .unwrap()
    //     .0 as i32

    // Custom solution without std libraries
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();

    for i in 0..haystack.len() {
        if needle_bytes[0] == haystack_bytes[i] {
            if needle_bytes.len() == 1 {
                return i as i32;
            }
            else {
                for j in 1..needle_bytes.len() {
                    if needle_bytes[j] != haystack_bytes[i + j] {
                        break;
                    } 
                    
                    // Check for last iteration
                    if j == (needle_bytes.len() - 1) {
                        return i as i32;
                    }
                }
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_first_occurrence() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    #[test]
    fn returns_minus_one_when_needle_is_missing() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn returns_the_first_index_for_repeated_matches() {
        assert_eq!(str_str("mississippi".to_string(), "iss".to_string()), 1);
    }

    #[test]
    fn finds_a_match_at_the_end_of_the_haystack() {
        assert_eq!(str_str("hello".to_string(), "lo".to_string()), 3);
    }
}
