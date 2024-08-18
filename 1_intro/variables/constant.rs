fn main() {
    // Constant must have static type
    // We cannot redeclare constants
    // For more information about constants evaluation:
    // https://doc.rust-lang.org/reference/const_eval.html
    const SECONDS_IN_HOUR: u16 = 60 * 60;
    println!("Seconds in hour {}", SECONDS_IN_HOUR);
}
