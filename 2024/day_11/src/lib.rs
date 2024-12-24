pub mod part_1;
pub mod part_2;

fn blink_times(stones: &mut Vec<usize>, times: usize) {
    for _ in 0..times {
        blink(stones);
    }
}

fn blink(stones: &mut Vec<usize>) {
    let mut i = 0;

    while i < stones.len() {
        match stones[i] {
            0 => stones[i] = 1,
            n if get_digits_count(n) % 2 == 0 => {
                let (left, right) = split_digits(n);

                stones.insert(i + 1, right);
                stones[i] = left;
                i += 1;
            }
            n => stones[i] = n * 2024,
        };

        i += 1;
    }
}

fn split_digits(n: usize) -> (usize, usize) {
    let digits = get_digits_count(n);

    let divisor = 10_usize.pow(digits as u32 / 2);

    let left = n / divisor;
    let right = n % divisor;

    (left, right)
}

fn get_digits_count(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as usize + 1
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}
