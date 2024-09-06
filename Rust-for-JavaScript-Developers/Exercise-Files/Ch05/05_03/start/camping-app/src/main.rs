use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("my_file.txt")
        .expect("Something went awry with the file");
    let text = " We're making it happen!";
    file.write_all(text.as_bytes())
        .expect("something went wrong");
}
