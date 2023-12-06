pub mod part_1;

fn get_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(parse_card)
}

fn get_card_points(c: Card) -> usize {
    let hits = c
        .numbers
        .iter()
        .filter(|n| c.winning_numbers.contains(n))
        .count();

    match hits {
        0 => 0,
        _ => 1 << (hits as u32 - 1),
    }
}

fn parse_card(input: &str) -> Card {
    let mut parts = input.split(':');

    let id_part = parts.next().expect("Should contain id part");

    let numbers_part = parts.next().expect("Should contain numbers part");

    let id = to_number(id_part);

    let (winning_numbers, numbers) = parse_numbers(numbers_part);

    Card {
        id,
        numbers,
        winning_numbers,
    }
}

fn parse_numbers(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut parts = input.split('|');

    let winning_numbers = parts.next().expect("Should contain winning numbers part");

    let numbers = parts.next().expect("Should contain a numbers part");

    let parse = |i: &str| i.split_whitespace().map(to_number).collect();

    (parse(winning_numbers), parse(numbers))
}

fn to_number(i: &str) -> usize {
    i.bytes()
        .filter(u8::is_ascii_digit)
        .map(|d| d as usize - 48)
        .rev()
        .enumerate()
        .map(|(i, d)| d * 10usize.pow(i as u32))
        .sum()
}

#[derive(Debug, PartialEq)]
struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
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

        let result = get_card_points(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn parse_line_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let result = parse_card(input);

        let expected = Card {
            id: 1,
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            winning_numbers: vec![41, 48, 83, 86, 17],
        };

        assert_eq!(result, expected);
    }
}
