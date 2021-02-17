fn main() {
    let some_u8_value = Some(0u8);

    /*
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }

        // alternative below
    */

    if let Some(3) = some_u8_value {
        println!("three");
    } else if let Some(0) = some_u8_value {
        println!("zero");
    } else {
        println!("some other value");
    }
}
