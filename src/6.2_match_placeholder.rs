fn main() {
    let some_u8_value = 0u8;

    match some_u8_value {
        0 => println!("zero"),
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (),
    }

    if some_u8_value == 0 {
        println!("value is 0");
    }
}
