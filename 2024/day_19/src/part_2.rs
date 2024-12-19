use std::collections::HashMap;

use crate::parse;

pub fn linen_layout(input: &str) -> usize {
    let (towels, designs) = parse(input);

    let mut memo = HashMap::new();

    designs
        .iter()
        .map(|d| doable_count(d, &towels, &mut memo))
        .sum()
}

fn doable_count(design: &str, towels: &[&str], memo: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = memo.get(design) {
        return count;
    }

    if design.is_empty() {
        return 1;
    }

    let mut count = 0;

    for part in towels {
        if let Some(remaining) = design.strip_prefix(part) {
            count += doable_count(remaining, towels, memo);
        }
    }

    memo.insert(design.to_owned(), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

        assert_eq!(16, linen_layout(input));
    }

    #[test]
    fn input() {
        let result = linen_layout(include_str!("input"));
        assert_eq!(848076019766013, result);
    }
}
