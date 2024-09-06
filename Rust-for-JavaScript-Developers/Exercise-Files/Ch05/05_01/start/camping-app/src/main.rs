struct Hiker {
    name: String,
    miles_hiked: u64,
}

fn main() {
    let jennifer = Hiker {
        name: String::from("Jennifer"),
        miles_hiked: 49,
    };
    println!("{} has hiked {} miles", jennifer.name, jennifer.miles_hiked);
}
