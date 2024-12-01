pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|l| l.split_whitespace())
        .map(|mut s| (s.next().expect("First"), s.next().expect("Second")))
        .map(|(first, second)| {
            (
                first.parse().expect("Number"),
                second.parse().expect("Number"),
            )
        })
        .fold((Vec::new(), Vec::new()), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);

            acc
        })
}
