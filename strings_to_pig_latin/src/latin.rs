// Exercise:
    // Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word 
    // and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the 
    // end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

enum Kind {
    Consonant,
    Vowel
}

pub fn string_to_pig_latin(input: &str) -> String {
    let mut output = String::new();

    for word in input.split(' ') {
        match get_kind_word(word) {
            Kind::Consonant => {
                output.push_str(get_consonant_pig_latin_word(word).as_str());
            },
            Kind::Vowel => {
                output.push_str(format!("{}-hay", word).as_str());
            },
        }
        // Add space in between words
        output.push(' ');
    }
    // Remove last space
    let trimmed_len = output.trim_end().len();
    output.truncate(trimmed_len);

    output
}

fn get_kind_word(word: &str) -> Kind {
    let first_letter = word.chars().next().unwrap(); // This might fail lol

    if first_letter == 'a' || first_letter == 'e' || first_letter == 'i' || first_letter == 'o' || first_letter == 'u' { 
        Kind::Vowel 
    } else {
        Kind::Consonant
    } 
}

fn get_consonant_pig_latin_word(word: &str) -> String {
    let mut consonant_pig_latin_word = String::new();
    let mut chars = word.chars();
    let first_char = chars.next().unwrap(); // We assume there are always words.

    for c in chars {
        // Middle characters go here
        consonant_pig_latin_word.push(c);
    }

    consonant_pig_latin_word.push_str(format!("-{}ay", first_char).as_str());

    consonant_pig_latin_word
}