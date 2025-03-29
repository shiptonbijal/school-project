use std::io;

fn main() {
    let input = String::new();
    println!("Enter some data:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<isize> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

    for &number in &numbers {
        if number % 2 == 0 {
            println!("This is an even number.");
        } else {
            println!("This is an odd number.");
        }
    }

    let sum = numbers.iter().sum::<isize>();
    println!("The sum of the numbers is: {}", sum);
}
