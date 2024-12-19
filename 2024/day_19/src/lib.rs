pub mod part_1;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = input.split("\n\n");

    let available_towels = parts.next().expect("Towels").split(", ").collect();

    let wanted_designs = parts.next().expect("Designs").lines().collect();

    (available_towels, wanted_designs)
}
