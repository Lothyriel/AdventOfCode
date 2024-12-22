pub mod part_1;
pub mod part_2;

fn generate_next(mut n: usize) -> usize {
    n = mix(n, n * 64);
    n = prune(n);
    n = mix(n, n / 32);
    n = prune(n);
    n = mix(n, n * 2048);
    n = prune(n);
    n
}

fn prune(n: usize) -> usize {
    n % 16777216
}

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().flat_map(|l| l.parse())
}
