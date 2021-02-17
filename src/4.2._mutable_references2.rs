fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // the alternative to the problem below:
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //  --> src/main.rs:5:14

    println!("{}, {}", r1, r2);
}
