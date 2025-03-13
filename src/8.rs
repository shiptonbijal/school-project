use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
