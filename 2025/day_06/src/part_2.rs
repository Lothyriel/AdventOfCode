use crate::{Operation, Problem};

pub fn trash_compactor(input: &str) -> usize {
    let rows: Vec<Vec<_>> = input.lines().map(|l| l.bytes().collect()).collect();

    let numbers_count = rows.len() - 1;
    let width = rows[0].len();

    let mut problems = Vec::new();
    let mut numbers = Vec::new();
    let mut op = None;

    for w in 0..width {
        match &rows[numbers_count][w] {
            b'*' => op = Some(Operation::Multiplication),
            b'+' => op = Some(Operation::Addition),
            _ => {}
        };

        let number = (0..numbers_count)
            .map(|n| rows[n][w])
            .filter(|c| c.is_ascii_digit())
            .fold(String::new(), |mut acc, d| {
                acc.push(d as char);

                acc
            });

        if number.is_empty() {
            let problem = Problem {
                operation: op.unwrap(),
                numbers: std::mem::take(&mut numbers),
            };
            problems.push(problem);
        } else {
            numbers.push(number.parse().unwrap());
        }
    }

    let problem = Problem {
        operation: op.unwrap(),
        numbers,
    };
    problems.push(problem);

    problems.iter().map(|p| p.solve()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

        let result = trash_compactor(input);
        assert_eq!(3263827, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = trash_compactor(input);
        assert_eq!(9348430857627, result);
    }
}
