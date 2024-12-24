use crate::{parse, Configuration};

pub fn claw_contraption(input: &str) -> isize {
    parse(input)
        .into_iter()
        .map(|c| c.move_prize(10000000000000))
        .filter_map(|c| c.min_tokens())
        .sum()
}

impl Configuration {
    fn move_prize(mut self, amount: isize) -> Configuration {
        self.prize = (self.prize.0 + amount, self.prize.1 + amount);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = claw_contraption(input);
        assert_eq!(83232379451012, result);
    }
}
