fn calculate_distance(days: u64, distance: u64) -> u64 {
    let total_miles = days * distance;
    return total_miles;
}

fn main() {
    let result = calculate_distance(5, 10);
    println!("{} miles", result);
}
