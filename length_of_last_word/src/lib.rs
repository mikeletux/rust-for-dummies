/// Returns the length of the last word in `s`.
///
/// A word is a maximal substring consisting only of non-space characters.
pub fn length_of_last_word(s: String) -> i32 {
    s
        .trim()
        .split(" ")
        .last()
        .unwrap_or("")
        .len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_last_word_without_trailing_spaces() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn ignores_trailing_spaces() {
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn handles_a_single_letter_last_word() {
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}
