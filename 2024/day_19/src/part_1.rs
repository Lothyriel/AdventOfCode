use crate::parse;

pub fn linen_layout(input: &str) -> usize {
    let (towels, designs) = parse(input);

    designs.iter().filter(|d| doable_design(d, &towels)).count()
}

fn doable_design(design: &str, towels: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }

    for part in towels {
        if let Some(stripped) = design.strip_prefix(part) {
            if doable_design(stripped, towels) {
                return true;
            }
        }
    }

    false
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

        assert_eq!(6, linen_layout(input));
    }

    #[test]
    fn input() {
        let result = linen_layout(include_str!("input"));
        assert_eq!(240, result);
    }
}
