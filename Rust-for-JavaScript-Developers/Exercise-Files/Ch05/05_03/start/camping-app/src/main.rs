fn main() {
    let destination = "Lakes";

    match destination {
        "Long Lake" => println!("The destination is Long Lake"),
        "Mammoth Lakes" => println!("The destination is Mammoth Lakes"),
        "Bowman Lake" => println!("The destination is Bowman Lake"),
        _ => println!("The destination is unknown"),
    };
}
