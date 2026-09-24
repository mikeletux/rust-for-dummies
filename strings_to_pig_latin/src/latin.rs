// Exercise:
    // Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word 
    // and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the 
    // end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

pub fn string_to_pig_latin(input: &str) -> String {
    let mut words = Vec::new();

    for word in input.split_whitespace() {
        words.push(word_to_pig_latin(word));
    }

    words.join(" ")
}

fn word_to_pig_latin(word: &str) -> String {
    let mut chars = word.chars(); // Iterator

    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    let rest_of_word = chars.as_str();

    if is_vowel(first_char) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", rest_of_word, first_char)
    }
}

fn is_vowel(c: char) -> bool {
    let vowels = "aeiouáéíóúAEIOUÁÉÍÓÚ";
    vowels.contains(c)
}