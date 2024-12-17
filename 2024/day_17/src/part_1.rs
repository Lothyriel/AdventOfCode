pub fn chronospatial_computer(input: &str) -> String {
    let mut program = crate::parse(input);

    program.execute();

    program.read_output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

        assert_eq!("4,6,3,5,6,3,5,2,1,0", chronospatial_computer(input));
    }

    #[test]
    fn input() {
        let expected = "6,0,6,3,0,2,3,1,6";
        assert_eq!(expected, chronospatial_computer(include_str!("input")));
    }
}
