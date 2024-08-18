use std::io;

fn is_even(number: i32) -> bool {
    return number % 2 == 0;
}

fn read_number() -> i32 {
    let mut input = String::new();

    println!("Enter your number");
    io::stdin()
        .read_line(&mut input)
        .expect("could not read input");

    let number: i32 = input
        .trim()
        .parse()
        .expect("could not convert input to number");

    return number;
}

fn main() {
    if is_even(read_number()) {
        println!("Number is even");
    } else {
        println!("Number is odd");
    }
}
