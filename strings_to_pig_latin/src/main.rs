mod latin;

fn main() {
    // Exercise:
    // Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word 
    // and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the 
    // end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
    
    // Single word starting with a consonant
    assert_eq!(latin::string_to_pig_latin("first"), String::from("irst-fay"));

    // Single word starting with a vowel
    assert_eq!(latin::string_to_pig_latin("apple"), String::from("apple-hay"));

    // Several words, mixing consonants and vowels
    assert_eq!(latin::string_to_pig_latin("hola alo"), String::from("ola-hay alo-hay"));

    // UTF-8: multi-byte first letter must be moved as a whole character
    assert_eq!(latin::string_to_pig_latin("ñandú"), String::from("andú-ñay"));

    // Longer sentence, mixing consonants and vowels
    assert_eq!(
        latin::string_to_pig_latin("the quick brown fox jumps over an old lazy dog"),
        String::from("he-tay uick-qay rown-bay ox-fay umps-jay over-hay an-hay old-hay azy-lay og-day")
    );

    // Longer sentence, all words starting with a vowel
    assert_eq!(
        latin::string_to_pig_latin("every orange is under an old umbrella"),
        String::from("every-hay orange-hay is-hay under-hay an-hay old-hay umbrella-hay")
    );

    // Longer sentence with UTF-8 characters in several positions
    assert_eq!(
        latin::string_to_pig_latin("ñoño come jamón y café en el año nuevo"),
        String::from("oño-ñay ome-cay amón-jay -yay afé-cay en-hay el-hay año-hay uevo-nay")
    );

    println!("All tests passed!");
}
