pub mod part_1;
pub mod part_2;

type Equation = (usize, Vec<usize>);

fn valid_equation(
    target: usize,
    numbers: &[usize],
    operators: &[fn(usize, usize) -> usize],
) -> bool {
    let (&first, numbers) = numbers.split_first().expect("At least two");

    check(first, target, numbers, operators)
}

fn check(
    current: usize,
    target: usize,
    numbers: &[usize],
    operators: &[fn(usize, usize) -> usize],
) -> bool {
    if current > target {
        return false;
    }

    let Some((&first, numbers)) = numbers.split_first() else {
        return current == target;
    };

    for op in operators {
        if check(op(current, first), target, numbers, operators) {
            return true;
        }
    }

    false
}

fn mul(a: usize, b: usize) -> usize {
    a * b
}

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");

            let target = parts.next().expect("Target").parse().expect("Valid number");

            let numbers = parts
                .next()
                .expect("Numbers")
                .split(' ')
                .flat_map(|n| n.parse())
                .collect();

            (target, numbers)
        })
        .collect()
}
