fn main() {
    struct Hiker {
        name: String,
        miles_hiked: u64,
    }

    let billy = Hiker {
        name: "Billy".to_string(),
        miles_hiked: 67,
    };

    let Hiker {name, miles_hiked} = billy;

    println!("name {}, miles: {}", name, miles_hiked);
}
