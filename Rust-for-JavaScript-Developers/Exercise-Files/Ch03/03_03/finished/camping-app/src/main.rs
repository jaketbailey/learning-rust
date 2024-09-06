fn calculate_distance(days: u64, distance: u64) -> Result<u64, String> {
    if days == 0 {
        Err("Cannot go on a zero day hike".to_string())
    } else {
        Ok(days * distance)
    }
}

fn main() {
    let result = calculate_distance(0, 10);
    match result {
        Ok(miles) => println!("Miles: {}", miles),
        Err(error) => println!("Error: {}", error),
    }
}
