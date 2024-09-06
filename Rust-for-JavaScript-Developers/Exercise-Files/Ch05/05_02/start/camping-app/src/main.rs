use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("my_file.txt")
        .expect("Could not open file");

    let text = " We're making it happen!";
    file.write_all(text.as_bytes())
        .expect("Could not write to file");
}
