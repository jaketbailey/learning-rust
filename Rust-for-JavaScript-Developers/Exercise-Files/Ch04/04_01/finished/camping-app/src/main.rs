fn sentence_builder(person_info: (&str, u64, &str)) {
    println!(
        "{} is {}, and her last initial is {}.",
        person_info.0, person_info.1, person_info.2
    );
}

fn main() {
    let values = ("Eve", 38, "P");
    sentence_builder(values);
}
