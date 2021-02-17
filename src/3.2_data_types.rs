fn main() {
    // 1. BASIC TYPES
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("The value of x is {:?}", x);
    println!("The value of y is {:?}", y);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;
    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "values 1-4: {:?} {:?} {:?} {:?}",
        sum, difference, quotient, remainder
    );

    // Boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("true, false, {:?}, {:?}", t, f);

    // the char type is 4 bytes in size, so read more in "Storing UTF-8 Encoded Text with Strings"
    // let c = 'z';
    // let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {}", heart_eyed_cat);

    // 2. COMPOUND TYPES

    // tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is {:?}, {:?}, {:?}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "The destructured values are {:?}, {:?}, {:?}",
        five_hundred, six_point_four, one
    );

    // array - every element must be of the same type
    // let a = [1, 2, 3, 4, 5];
    // let first_months = ["January", "February", "March"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    println!("a[2] = {}", a[2]);
    // 5 elements, all set to the number 3
    let a = [3; 5];
    println!("a = {:?}", a);
}
