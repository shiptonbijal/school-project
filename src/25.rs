fn main() {
    // Example of an iterator that yields each element in a vector one at a time.
    let items = vec![1, 2, 3, 4, 5];
    for &item in &items {
        println!("{}", item);
    }
}
