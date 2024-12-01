pub fn get_total_cards(input: &str) -> usize {
    let card_points: Vec<_> = crate::parse_cards(input)
        .map(|c| (c.id, c.get_matching().count()))
        .collect();

    card_points
        .iter()
        .map(|c| get_cards_count(&card_points, c))
        .sum()
}

fn get_cards_count(card_points: &[(usize, usize)], card: &(usize, usize)) -> usize {
    let (id, matches) = card;

    let copies: usize = card_points
        .iter()
        .skip(*id)
        .take(*matches)
        .map(|c| get_cards_count(card_points, c))
        .sum();

    1 + copies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let result = get_total_cards(input);

        assert_eq!(result, 30);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let result = get_total_cards(input);

        assert_eq!(result, 5833065);
    }
}
