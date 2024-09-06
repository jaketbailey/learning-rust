#[derive(Debug)]
enum Steepness {
    Easy,
    Moderate,
    Difficult,
}

fn main() {
    let _calm_trail = Steepness::Easy;
    let _moderate_slope = Steepness::Moderate;
    let prickly_peak_trail = Steepness::Difficult;
    println!("Steepness is {:?}", prickly_peak_trail);
}
