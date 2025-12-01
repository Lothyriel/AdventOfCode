use crate::{Dial, parse};

pub fn secret_entrance(input: &str) -> isize {
    let mut dial = Dial::new();

    parse(input).fold(0, |count, r| {
        dial.apply(&r);

        if dial.pos == 0 { count + 1 } else { count }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        let result = secret_entrance(input);
        assert_eq!(3, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = secret_entrance(input);
        assert_eq!(1195, result);
    }
}
