fn main() {
    let farmer = std::sync::Arc::new(day_05::FarmerAlmanac::parse(include_str!("input")));

    let start = std::time::SystemTime::now();

    let result = farmer.get_lowest_location_range();

    let duration = start.elapsed().expect("Should get elapsed time");

    println!("Lowest soil: {}", result);

    println!("Time solving: {:?}", duration);
}
