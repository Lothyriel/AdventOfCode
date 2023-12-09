use std::cmp::Ordering;

pub fn get_total_winnings(input: &str) -> usize {
    let mut games: Vec<_> = crate::parse_games(input).collect();

    games.sort_by(|g1, g2| {
        let ord = g1.hand.cmp(&g2.hand);

        match ord {
            Ordering::Equal => g1.break_tie(g2),
            _ => ord,
        }
    });

    games.iter().enumerate().map(|(i, g)| g.bid * (i + 1)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result = get_total_winnings(include_str!("example"));

        assert_eq!(result, 6440);
    }

    #[test]
    fn puzzle() {
        let result = get_total_winnings(include_str!("input"));

        assert_eq!(result, 250370104);
    }
}
