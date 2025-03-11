use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    let mut map = HashMap::new();
    let file_path = Path::new("data.txt");
    let file = File::open(file_path).unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut fields = line.split(",");
        let key = fields.next().unwrap();
        let value = fields.next().unwrap();
        map.insert(key, value);
    }
}
