#[derive(Debug)]
enum Steepness {
    Easy,
    Moderate,
    Difficult,
}

fn main() {
    let _calm_trail = Steepness::Easy;
    let _fun_trail = Steepness::Moderate;
    let prickly_peak_trail = Steepness::Difficult;
    println!("Steepness is {:?}", prickly_peak_trail);
}
