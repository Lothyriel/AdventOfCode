use std::cmp::Ordering;

pub fn get_total_winnings(input: &str) -> usize {
    let mut games: Vec<_> = crate::parse_games(input).collect();

    change_jester(&mut games);

    games.sort_by(|g1, g2| {
        let ord = g1.hand.value().cmp(&g2.hand.value());

        match ord {
            Ordering::Equal => g1.break_tie_jester(g2),
            _ => ord,
        }
    });

    games.iter().enumerate().map(|(i, g)| g.bid * (i + 1)).sum()
}

fn change_jester(games: &mut [crate::Game]) {
    for game in games {}
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
