fn main() {
    // we can redeclare variables in signle scope
    // first x is immutable
    let x = 1;
    // second x is mutable, and after this line we can't access first x,
    // 'cause first variable is shadowed
    let mut x = x + 2 * 2;

    // scope visibility (like in ecmascript)
    {
        let x = x * 10;
        println!("Scoped variable value: {}", x);
    }

    x = x + 1;
    println!("Variable x has value: {}", x);

    // we can change variable type
    let x = "I'm new type value!";
    println!("Variable x has value: {}", x);

    // but we can't change variable type on the fly
    // like this: x = x.len()
    // this will cause an error
}
