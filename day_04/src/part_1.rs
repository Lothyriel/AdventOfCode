use crate::{parse_cards, Card};

pub fn get_points(input: &str) -> usize {
    parse_cards(input).map(|c| get_card_points(&c)).sum()
}

fn get_card_points(c: &Card) -> usize {
    let hits = c.get_matching().count();

    match hits {
        0 => 0,
        _ => 1 << (hits as u32 - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_points_line_1() {
        let input = Card {
            id: 1,
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            winning_numbers: vec![41, 48, 83, 86, 17],
        };

        let result = get_card_points(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn example() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let result = get_points(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let result = get_points(input);

        assert_eq!(result, 20667);
    }
}
