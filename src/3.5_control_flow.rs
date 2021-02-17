fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // -----------------------------------------

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // wohoo loops can return a result
        }
    };

    println!("The result is {}", result);

    // -----------------------------------------

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // -----------------------------------------

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {};", a[index]);

        index += 1;
    }

    // -----------------------------------------

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // -----------------------------------------

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
