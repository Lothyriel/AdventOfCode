use std::cmp::Ordering;

use crate::{Card, Hand};

fn parse_game_jester(input: &str) -> crate::Game {
    let mut game = crate::Game::parse(input);

    let jesters = game.cards.iter().filter(|c| matches!(c, Card::J)).count();

    change_jester(&mut game, jesters);

    game
}

pub fn get_total_winnings(input: &str) -> usize {
    let mut games: Vec<_> = input.lines().map(parse_game_jester).collect();

    games.sort_by(|g1, g2| {
        let ord = g1.hand.cmp(&g2.hand);

        match ord {
            Ordering::Equal => g1.break_tie_jester(g2),
            _ => ord,
        }
    });

    games.iter().enumerate().map(|(i, g)| g.bid * (i + 1)).sum()
}

fn change_jester(game: &mut crate::Game, count: usize) {
    if count == 0 {
        return;
    }

    if game.hand == Hand::One && count == 2 {
        game.hand = Hand::Three;
        return;
    }

    if game.hand == Hand::Three && count == 3 {
        game.hand = Hand::Four;
        return;
    }

    game.hand = match game.hand {
        Hand::High => Hand::One,
        Hand::One => Hand::Three,
        Hand::Two => Hand::Full,
        Hand::Three => Hand::Four,
        Hand::Full => Hand::Four,
        Hand::Four | Hand::Five => Hand::Five,
    };

    change_jester(game, count - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jester() {
        let test = |input: &str| parse_game_jester(input).hand;

        assert_eq!(test("J8787 166"), Hand::Full);
        assert_eq!(test("2J957 35"), Hand::One);

        assert_eq!(test("Q2KJJ 13"), Hand::Three);
        assert_eq!(test("T3T3J 17"), Hand::Full);
        assert_eq!(test("2345J 3"), Hand::One);
        assert_eq!(test("J345A 2"), Hand::One);
        assert_eq!(test("T55J5 29"), Hand::Four);
        assert_eq!(test("KTJJT 34"), Hand::Four);
        assert_eq!(test("JJJJJ 37"), Hand::Five);
        assert_eq!(test("AAAAJ 59"), Hand::Five);
        assert_eq!(test("2JJJJ 53"), Hand::Five);
        assert_eq!(test("JJJJ2 41"), Hand::Five);

        assert_eq!(test("2233J 1"), Hand::Full);
        assert_eq!(test("JJJ34 1"), Hand::Four);
    }

    #[test]
    fn reddit_example() {
        let result = get_total_winnings(include_str!("reddit_input"));

        assert_eq!(result, 6839);
    }

    #[test]
    fn example() {
        let result = get_total_winnings(include_str!("example"));

        assert_eq!(result, 5905);
    }

    #[test]
    fn puzzle() {
        let result = get_total_winnings(include_str!("input"));

        assert_eq!(result, 251735672);
    }
}
