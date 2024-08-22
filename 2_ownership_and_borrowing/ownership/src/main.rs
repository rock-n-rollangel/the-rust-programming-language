
use std::io;
/*
    Ownership Rules:
    - Each value in Rust has an owner
    - There can be only one owner at a time
    - When the owner goes out of scope, the value will be dropped
*/

fn main() {
    let some_string_1 = String::from("Hello, world!");
    let some_string_2 = takes_ownership(some_string_1);
    // variable some_string_1 not valid after that line,
    // but we have same (see function) variables ownership back
    println!("{some_string_2}"); // if we try to print some_string_1 we'll receive an error

    let some_int: u8 = 13;
    makes_copy(some_int);
    println!("{some_int}");
}

fn five() -> i8 {
    // variable five only valid inside this function.
    // this function is the scope of `five`.
    // data saved to the stack.
    let five: i8 = 5;
    five
}

fn read_input() -> String {
    // this will be stored in the heap, 'cause we don't know size
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("could not read input");

    input
}

fn takes_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string // gives ownership back
}

fn makes_copy(some_int: u8) {
    println!("{some_int}");
}
