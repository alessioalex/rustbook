fn main() {
    // this reference can live for the entire duration of the program;
    // all string literals have the 'static lifetime
    let s: &'static str = "I have a static lifetime.";
}
