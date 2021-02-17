fn main() {
    let s = String::from("hello world");
    // let s = "hello world";

    let hello = &s[0..5];
    let hello2 = &s[..5];
    // let world = &s[6..11];
    // let world2 = &s[6..];
    let world3 = &s[6..s.len()];
    // [starting_index, ending_index)

    println!("{} {} {}", hello, hello2, world3);
}
