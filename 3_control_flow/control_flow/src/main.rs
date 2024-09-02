
fn main() {
    // let a: isize = 124;
    // Compiler give us error, 'cause `a` isn't bool type
    // if a {
    //     println!("Variable a must be boolean")
    // }
    //
    // if is_odd(a) {
    //     println!("Number is odd")
    // } else {
    //     println!("Number is even")
    // }

    // let b = if is_odd(a) { 1 } else { 0 };
    // println!("b is {b}")

    // This code will cause an error, 'cause of types
    // each arm of if statement must have same types.
    // let cond = true;
    // let b = if cond { 5 } else { "five" }

    let message = String::from("Hello");
    print_times(message, 5);

    println!("odds in number: {}", count_odds(123));
    let collection: [String; 5] = [
        String::from("asdf 1"),
        String::from("asdf 2"),
        String::from("asdf 3"),
        String::from("asdf 4"),
        String::from("asdf 5")
    ];
    print_collection(collection);
}

fn is_odd(number: isize) -> bool {
    number % 2 > 0
}

fn print_times(message: String, times: usize) {
    let mut mt = times;
    while mt != 0 {
        println!("{message}");
        mt -= 1;
    }
}

fn count_odds(number: isize) -> usize {
    let mut odds: usize = 0;
    let mut mt = number;
    // We can use labeled loops. It's helpful with multiple loops.
    'counting_odds: loop {
        if is_odd(mt) {
            odds += 1;
        }

        mt -= 1;

        if mt == 0 {
            // Here we tell what loop to break by label.
            // We can use continue as well.
            break 'counting_odds odds;
        }
    }
}

fn print_collection(collection: [String; 5]) {
    // This is how to loop over collection with `while`
    // let mut i: usize = 0;
    // while i < 5 {
    //     println!("Collection item with index {i} is {}", collection[i]);
    //     i += 1;
    // }

    // But with for this is more clear
    for element in collection {
        println!("{element}")
    }
}
