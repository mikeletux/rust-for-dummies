pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut stack: Vec<u8> = Vec::with_capacity(s.len()/2); // Allocate memory in the HEAP soon
    for c in s.bytes() { // Pure ASCII string, so it's faster
        match c {
            b'(' => stack.push(b')'),
            b'{' => stack.push(b'}'),
            b'[' => stack.push(b']'),
            _ => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
        }
    }
        stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_valid() {
        let test1 = String::from("()");
        assert!(is_valid(test1));

        let test2 = String::from("()[]{}");
        assert!(is_valid(test2));

        let test3 = String::from("(]");
        assert!(!is_valid(test3));

        let test4 = String::from("([])");
        assert!(is_valid(test4));

        let test5 = String::from("([)]");
        assert!(!is_valid(test5));
    }
}
