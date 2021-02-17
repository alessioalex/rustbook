fn main() {
    let sentence = String::from("hello world");

    let index = first_word(&sentence);

    println!(
        "The first word in '{}' starts at the {}th letter.",
        sentence, index
    );
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
