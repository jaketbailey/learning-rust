fn main() {
    let destination = "Snow Lake";

    match destination {
        "Long Lake" => println!("We're heading to Long Lake!"),
        "Mammoth Lakes" => println!("We're heading to Mammoth!"),
        "Bowman Lake" => println!("We're heading to Bowman Lake!"),
        _ => println!("We're heading anywhere else"),
    }
}
