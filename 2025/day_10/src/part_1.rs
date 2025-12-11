use crate::{LightState, Machine};
use lib::all_combinations;

pub fn factory(input: &str) -> usize {
    input.lines().map(Machine::parse).map(|m| least_buttons(&m)).sum()
}

fn least_buttons(machine: &Machine) -> usize {
    let combinations = all_combinations(&machine.buttons);

    let valid = combinations.iter().filter(|c| {
        c.iter().fold(vec![LightState::Off; machine.final_config.len()], |mut acc, b| {
            b.press(&mut acc);

            acc
        }) == machine.final_config
    });

    valid.map(|b| b.len()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        let result = factory(input);
        assert_eq!(7, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = factory(input);
        assert_eq!(520, result);
    }
}