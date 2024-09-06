fn main() {
    let days: u64 = 5;
    println!("{} days", days);
    let backpack_weight: f32 = 28.5;
    println!(
        "hiking for {} days with a {} lb. pack",
        days, backpack_weight
    );
    let meals = days * 3;
    println!("{} meals", meals);
}
