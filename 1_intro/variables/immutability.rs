fn main() {
    let x = 5;
    println!("Value of x is {}", x);

    // Changing value of immutable variable will fail with error
    x = 6;
    println!("Value of x is changed to {}", x);
}
