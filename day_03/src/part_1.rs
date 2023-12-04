pub fn get_schematic_value(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let result = get_schematic_value(input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn puzzle() {
        let input = include_str!("input");

        let result = get_schematic_value(input);

        assert_eq!(result, 0);
    }
}
