use std::fs;

fn main() {
    let mut file = fs::File::open("hello_world.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    println!("{}", contents);
}
