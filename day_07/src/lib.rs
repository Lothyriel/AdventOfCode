use std::{cmp::Ordering, collections::HashMap};

pub mod part_1;

fn parse_games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(Game::parse)
}

#[derive(Debug)]
struct Game {
    bid: usize,
    hand: Hand,
    cards: Vec<Card>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let mut parts = input.split_whitespace();

        let cards: Vec<_> = parts
            .next()
            .expect("Expected hands part")
            .bytes()
            .flat_map(Card::try_parse)
            .collect();

        let bid = parts
            .next()
            .expect("Expected bid part")
            .parse()
            .expect("Expected a number");

        Self {
            bid,
            hand: Hand::parse(cards.iter()),
            cards,
        }
    }

    fn break_tie(&self, other: &Self) -> Ordering {
        let diff = self
            .cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(c1, c2)| match c1.cmp(c2) {
                Ordering::Equal => None,
                c => Some(c),
            });

        diff.unwrap_or(Ordering::Equal)
    }
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Number(usize),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn try_parse(card: u8) -> Option<Self> {
        match card {
            b'A' => Some(Card::A),
            b'K' => Some(Card::K),
            b'Q' => Some(Card::Q),
            b'J' => Some(Card::J),
            b'T' => Some(Card::T),
            b'2'..=b'9' => Some(Card::Number(card as usize)),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Hand {
    High(Card),
    One(Card),
    Two(Card, Card),
    Three(Card),
    Full(Card, Card),
    Four(Card),
    Five(Card),
}

impl Hand {
    fn value(&self) -> usize {
        match self {
            Hand::High(_) => 1,
            Hand::One(_) => 2,
            Hand::Two(_, _) => 3,
            Hand::Three(_) => 4,
            Hand::Full(_, _) => 5,
            Hand::Four(_) => 6,
            Hand::Five(_) => 7,
        }
    }
}

impl Hand {
    fn parse<'a>(cards: impl Iterator<Item = &'a Card>) -> Hand {
        let cards = cards.fold(HashMap::new(), |mut acc, c| {
            let entry = acc.entry(*c).or_insert(0usize);

            *entry += 1;

            acc
        });

        let (&card, max) = cards
            .iter()
            .max_by(|(_, a1), (_, a2)| a1.cmp(a2))
            .expect("Should have a max number");

        match max {
            5 => Hand::Five(card),
            4 => Hand::Four(card),
            3 => {
                let second = cards.iter().find(|(_, &count)| count == 2);

                match second {
                    Some((&second, _)) => Hand::Full(card, second),
                    None => Hand::Three(card),
                }
            }
            2 => {
                let mut pairs = cards.iter().filter(|(_, &count)| count == 2);

                let (&first, _) = pairs.next().expect("Expected a pair");

                match pairs.next() {
                    Some((&second, _)) => Hand::Two(first.max(second), first.min(second)),
                    None => Hand::One(first),
                }
            }
            1 => Hand::High(*cards.keys().max().expect("Should have max")),
            _ => unreachable!("Can't have 6 cards"),
        }
    }
}
