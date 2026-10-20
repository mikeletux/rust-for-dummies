//! # 884. Uncommon Words from Two Sentences
//!
//! A sentence is a string of single-space-separated words. Each word consists
//! only of lowercase English letters.
//!
//! A word is uncommon if it appears exactly once in one sentence and does not
//! appear in the other sentence.
//!
//! Given two sentences, `s1` and `s2`, return all uncommon words in any order.
//!
//! ## Examples
//!
//! - `s1 = "this apple is sweet"`, `s2 = "this apple is sour"` returns
//!   `["sweet", "sour"]`.
//! - `s1 = "apple apple"`, `s2 = "banana"` returns `["banana"]`.
//!
//! ## Constraints
//!
//! - `1 <= s1.len(), s2.len() <= 200`
//! - Both sentences contain only lowercase English letters and spaces.
//! - Neither sentence has leading or trailing spaces.
//! - Words are separated by exactly one space.
//!
//! Source: <https://leetcode.com/problems/uncommon-words-from-two-sentences/>


use std::collections::HashMap;

/// Returns the words that occur exactly once across both sentences, in any order.
pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut words: HashMap<String,bool> = HashMap::new(); // If the word is repeated
    let mut uncommon_words: Vec<String> = Vec::new();

    // Lets append both into a big string (by adding a " ") and then parse and add to a hashmap
    for word in (s1 + " " + &s2).split(' ') {
        words
            .entry(word.to_string())
            .and_modify(|e| *e = true)
            .or_insert(false);
    }

    for (word, exist) in words {
        if !exist {
            uncommon_words.push(word);
        }
    }

    uncommon_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_returns_sweet_and_sour() {
        let mut result = uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        result.sort();

        assert_eq!(result, vec!["sour".to_string(), "sweet".to_string()]);
    }

    #[test]
    fn example_2_returns_only_banana() {
        let result = uncommon_from_sentences("apple apple".to_string(), "banana".to_string());

        assert_eq!(result, vec!["banana".to_string()]);
    }
}
