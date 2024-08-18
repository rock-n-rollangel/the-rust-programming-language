fn main() {
    // mut suffix make this variable mutable
    // this make variable read-write, so we can update value
    let mut x = 5;
    println!("Value inside x is {}", x);

    x = x * 10;
    println!("Updated value inside x is {}", x);
}
