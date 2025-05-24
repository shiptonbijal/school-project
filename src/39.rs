fn main() {
    // Example Rust program that calculates the factorial of a number.
    let num = 5;
    match num {
        0 => println!("Factorial: {}", (1..num).product()),
        _ => println!("Factorial: {}", num),
    }
}
