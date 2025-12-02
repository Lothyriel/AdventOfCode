pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> impl Iterator<Item = Range> {
    input.trim().split(',').map(|r| {
        let mut parts = r.split('-');

        let first_id = parts.next().unwrap().parse().unwrap();

        let last_id = parts.next().unwrap().parse().unwrap();

        Range { first_id, last_id }
    })
}

#[derive(Debug)]
struct Range {
    first_id: usize,
    last_id: usize,
}
