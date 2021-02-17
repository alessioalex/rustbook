fn main() {
    let sentence = String::from("hello world");
    let first_word_in_sentence = first_word(&sentence);

    // the following works too, because &str allows both &String (reference to a string)
    // and &str (string slice)
    // let sentence = "hello world";
    // let first_word_in_sentence = first_word(sentence);

    // the compiler does not let us empty (mutate) the string
    // because we have a slice (immutable) of it
    // sentence.clear();

    println!("first_word_in_sentence = '{}'", first_word_in_sentence);
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
