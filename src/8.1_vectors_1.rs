fn main() {
    // let v = vec![1, 2, 3];
    // let v: Vec<i32> = Vec::new();

    let mut v = Vec::new();

    // type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation
    v.push(5);
    v.push(6);
    v.push(7);

    println!("{:#?}", v);

    // reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The 3rd element is {}", third);

    match v.get(2) {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("There is no 3rd element"),
    }

    if let Some(&fourth) = v.get(3) {
        println!("fourth is {}", fourth);
    }
}
