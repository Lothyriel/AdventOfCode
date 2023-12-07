fn main() {
    let farmer = day_05::FarmerAlmanac::parse_2(include_str!("input"));

    let start = std::time::SystemTime::now();

    let result = farmer.get_lowest_location();

    let duration = start.elapsed().expect("Should get elapsed time");

    println!("Seeds count: {:?}", farmer.seeds.len());

    println!("Lowest soil: {}", result);

    unsafe {
        println!("Comparisons: {}", day_05::COMPARISONS);
        println!("Calculations: {}", day_05::CALCULATIONS);
    }

    println!("Time: {:?}", duration);
}
