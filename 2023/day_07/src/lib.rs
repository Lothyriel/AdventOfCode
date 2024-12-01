use std::{cmp::Ordering, collections::HashMap};

pub mod part_1;
pub mod part_2;

fn parse_games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(Game::parse)
}

fn parse_game(input: &str) -> (Vec<Card>, usize) {
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

    (cards, bid)
}

#[derive(Debug)]
struct Game {
    bid: usize,
    hand: Hand,
    cards: Vec<Card>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let (cards, bid) = parse_game(input);

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

    fn break_tie_jester(&self, other: &Self) -> Ordering {
        let diff = self
            .cards
            .iter()
            .zip(other.cards.iter())
            .find_map(|(c1, c2)| match c1.value_jester().cmp(&c2.value_jester()) {
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
            b'2'..=b'9' => Some(Card::Number(card as usize - 48)),
            _ => None,
        }
    }

    fn value_jester(&self) -> usize {
        match self {
            Card::J => 1,
            Card::Number(n) => *n,
            Card::T => 10,
            Card::Q => 11,
            Card::K => 12,
            Card::A => 13,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl Hand {
    fn parse<'a>(cards: impl Iterator<Item = &'a Card>) -> Hand {
        let cards = cards.fold(HashMap::new(), |mut acc, c| {
            let entry = acc.entry(*c).or_insert(0usize);

            *entry += 1;

            acc
        });

        let (_, max) = cards
            .iter()
            .max_by(|(_, a1), (_, a2)| a1.cmp(a2))
            .expect("Should have a max number");

        match max {
            5 => Hand::Five,
            4 => Hand::Four,
            3 => {
                let second = cards.iter().find(|(_, &count)| count == 2);

                match second {
                    Some(_) => Hand::Full,
                    None => Hand::Three,
                }
            }
            2 => {
                let mut pairs = cards.iter().filter(|(_, &count)| count == 2);

                let _ = pairs.next().expect("Expected a pair");

                match pairs.next() {
                    Some(_) => Hand::Two,
                    None => Hand::One,
                }
            }
            1 => Hand::High,
            _ => unreachable!("Can't have 6 cards"),
        }
    }
}
