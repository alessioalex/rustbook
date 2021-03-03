use std::iter::FromIterator;
/*
Convert strings to pig latin.
The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the
end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn convert_string_to_pig_latin(str: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    // let mut letters: Vec<char> = str.chars().collect();
    let letters: Vec<char> = str.chars().collect();

    if let Some(first_letter) = letters.first() {
        if VOWELS.contains(&first_letter.to_ascii_lowercase()) {
            return format!("{}-hay", String::from_iter(&letters));
        } else {
            // let first_letter = letters.remove(0);
            // return format!("{}-{}ay", String::from_iter(&letters), first_letter);
            return format!(
                "{}-{}ay",
                String::from_iter(&letters[first_letter.len_utf8()..]),
                first_letter
            );
        }
    }

    str.to_string()
}

fn main() {
    // let's assume unicode
    let word = "first";
    println!("{} => {}", word, convert_string_to_pig_latin(word));
    let word = "Apples and oranges I like";
    println!("{} => {}", word, convert_string_to_pig_latin(word));
}
