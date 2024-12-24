use std::collections::HashMap;

use crate::parse;

pub fn plutonian_pebbles(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|&s| expand(s, 75, &mut HashMap::new()))
        .sum()
}

fn expand(stone: usize, blink_count: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if blink_count == 0 {
        return 1;
    }

    let k = (stone, blink_count);

    if let Some(&expanded_len) = memo.get(&k) {
        expanded_len
    } else {
        let count = blink(stone)
            .iter()
            .map(|&s| expand(s, blink_count - 1, memo))
            .sum();

        memo.insert(k, count);
        count
    }
}

fn blink(stone: usize) -> Vec<usize> {
    let mut stones = vec![stone];
    crate::blink(&mut stones);
    stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = plutonian_pebbles(input);
        assert_eq!(240884656550923, result);
    }
}
