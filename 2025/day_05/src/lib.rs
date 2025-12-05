pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> (Vec<Range>, Vec<usize>) {
    let mut parts = input.trim().split("\n\n");

    let fresh = parts.next().unwrap().split('\n').map(|r| {
        let mut parts = r.split('-');

        let start = parts.next().unwrap().parse().unwrap();

        let end = parts.next().unwrap().parse().unwrap();

        Range { start, end }
    });

    let ingredients = parts
        .next()
        .unwrap()
        .split('\n')
        .map(|l| l.parse().unwrap());

    (fresh.collect(), ingredients.collect())
}

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}
