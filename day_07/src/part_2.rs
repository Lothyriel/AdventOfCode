use std::cmp::Ordering;

use crate::{Card, Hand};

pub fn get_total_winnings(input: &str) -> usize {
    let mut games: Vec<_> = crate::parse_games(input).collect();

    games
        .iter_mut()
        .map(|g| (g.cards.iter().filter(|c| matches!(c, Card::J)).count(), g))
        .for_each(|(c, g)| change_jester(g, c));

    games.sort_by(|g1, g2| {
        let ord = g1.hand.value().cmp(&g2.hand.value());

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
        Hand::High(c) => Hand::One(c),
        Hand::One(c) => Hand::Two(c, *game.cards.first().expect("Should have card")),
        Hand::Two(c1, c2) => Hand::Full(c1, c2),
        Hand::Three(c) => Hand::Four(c),
        Hand::Full(c, _) => Hand::Four(c),
        Hand::Four(c) | Hand::Five(c) => Hand::Five(c),
    };

    change_jester(game, count - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_total_winnings(include_str!("example"));

        assert_eq!(result, 5905);
    }

    #[test]
    fn puzzle() {
        let result = get_total_winnings(include_str!("input"));

        assert_eq!(result, 0);
    }
}
