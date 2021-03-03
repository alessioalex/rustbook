fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // &hello[0] or &hello[0..4] not ok
    // let hello = "Здравствуйте";
    let hello = "नमस्ते";
    // ..instead use:
    for b in hello.chars() {
        print!("{} ", b);
    }
    print!("\n");

    // or for raw bytes
    for b in hello.bytes() {
        print!("{} ", b);
    }
    print!("\n");

    // Getting grapheme clusters (letters) from strings is complex,
    // so this functionality is not provided by the standard library.
}
