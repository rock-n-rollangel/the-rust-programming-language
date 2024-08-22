
/*
Let’s recap what we’ve discussed references:

    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.
*/

fn main() {
    let s = String::from("Hello, world!");
    let sl = calculate_length(&s);

    println!("length of {s} is {sl}");


    // We can have only one mutable reference at a time
    let mut ss = String::from("Hello, Mutability!");
    modify_string_ref(&mut ss);

    println!("{ss}");


    let r1 = &ss;
    let r2 = &ss;
    // let r3 = &mut ss; this will cause an error, 'cause we can't have a mutable refs while having refs
    println!("{r1} {r2}");


    let r3 = &mut ss;
    println!("{r3}"); // but we can use mutable ref here, 'cause others out of scope here!
}

// we can't modify, 'cause of immutability
// trying to modify will cause an error
fn calculate_length(s: &String) -> usize {
    s.len()
}

// we can modify s here, 'cause we tell that variable is mutable
fn modify_string_ref(s: &mut String) {
    s.push_str(" ! sdf");
}

// we can't make dangling refs
// this code will cause an error
// fn dangle() -> &String {
//     let s = String::from("sdf");
//     &s
// }
