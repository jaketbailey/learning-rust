use std::fs;

fn main() {
    let text = fs::read_to_string("my_file.txt").expect("Something went wrong reading the file");

    println!("What is in this file:\n{}", text);
}
