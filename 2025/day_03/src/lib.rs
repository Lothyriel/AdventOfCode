pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> impl Iterator<Item = Bank> {
    input.lines().map(|l| Bank {
        batteries: l.bytes().map(|b| b - b'0').collect(),
    })
}

fn get_largest_number(batteries: &[u8], n: usize) -> usize {
    get_largest_digits(batteries, n)
        .into_iter()
        .fold(0, |acc, d| acc * 10 + d as usize)
}

fn get_largest_digits(bats: &[u8], n: usize) -> Vec<u8> {
    let mut drops = bats.len() - n;
    let mut digits = Vec::with_capacity(n);

    for &d in bats {
        while digits.pop_if(|x| *x < d && drops > 0).is_some() {
            drops -= 1;
        }
        digits.push(d);
    }

    digits.truncate(n);
    digits
}

#[derive(Debug)]
struct Bank {
    batteries: Vec<u8>,
}
