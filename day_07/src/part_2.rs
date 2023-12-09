use std::cmp::Ordering;

use crate::{Card, Hand};

pub fn get_total_winnings(input: &str) -> usize {
    let mut games: Vec<_> = crate::parse_games(input).collect();

    games
        .iter_mut()
        .map(|g| (g.cards.iter().filter(|c| matches!(c, Card::J)).count(), g))
        .for_each(|(c, g)| change_jester(g, c));

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

    game.hand = match game.hand {
        Hand::High => Hand::One,
        Hand::One => Hand::Two,
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
        let test = |input: &str| {
            let mut game = crate::Game::parse(input);
            let jester_count = game.cards.iter().filter(|c| matches!(c, Card::J)).count();
            change_jester(&mut game, jester_count);
            game.hand
        };

        assert_eq!(test("J8787 166"), Hand::Full);
        assert_eq!(test("2J957 35"), Hand::One);
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

    // #[test]
    // fn puzzle() {
    //     let result = get_total_winnings(include_str!("input"));
    //
    //     assert_eq!(result, 0);
    // }
}
