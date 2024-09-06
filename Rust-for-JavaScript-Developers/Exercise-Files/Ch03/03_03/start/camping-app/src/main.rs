fn calculate_distance(days: u64, distance: u64) -> Result<u64, String> {
    if days == 0 {
        return Err("Days cannot be zero".to_string());
    } else {
        Ok(days * distance)
    }
}

fn main() {
    let result = calculate_distance(5, 10);
    match result {
        Ok(total_miles) => println!("Total miles: {}", total_miles),
        Err(error) => println!("Error: {}", error),
    }
}
