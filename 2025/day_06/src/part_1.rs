use crate::{Operation, Problem};

pub fn trash_compactor(input: &str) -> usize {
    let rows: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.trim()).collect())
        .collect();

    let cols = (0..rows[0].len()).map(|i| rows.iter().map(|row| row[i]).collect::<Vec<_>>());

    let problems = cols.map(|c| {
        let operation = match c.last().unwrap() {
            &"*" => Operation::Multiplication,
            &"+" => Operation::Addition,
            op => panic!("Invalid op {op}"),
        };

        let numbers = c.iter().filter_map(|n| n.parse().ok()).collect();

        Problem { operation, numbers }
    });

    problems.map(|p| p.solve()).sum()
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
        assert_eq!(4277556, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = trash_compactor(input);
        assert_eq!(4449991244405, result);
    }
}
