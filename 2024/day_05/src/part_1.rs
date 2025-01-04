pub fn print_queue(input: &str) -> usize {
    let (rules, updates) = crate::parse(input);

    updates
        .iter()
        .filter(|update| update.is_sorted_by(|x, y| !rules.contains(&(*y, *x))))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let result = print_queue(input);
        assert_eq!(143, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = print_queue(input);
        assert_eq!(7365, result);
    }
}
