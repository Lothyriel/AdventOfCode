pub fn chronospatial_computer(input: &str) -> isize {
    let mut prog = crate::parse(input);
    let expected: Vec<_> = prog.instructions.iter().map(|i| *i as isize).collect();

    let mut factors = vec![0; prog.instructions.len()];

    loop {
        let mut possible_a = 0;

        for (i, f) in factors.iter().enumerate() {
            possible_a += 8isize.pow(i as u32) * f
        }

        prog.a = possible_a;
        prog.b = 0;
        prog.c = 0;
        prog.pointer = 0;
        prog.output.clear();
        prog.execute();

        if prog.output == expected {
            return possible_a;
        }

        for i in (0..prog.instructions.len()).rev() {
            if prog.output.get(i).copied() != Some(prog.instructions[i] as isize) {
                factors[i] += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

        assert_eq!(117440, chronospatial_computer(input));
    }

    #[test]
    fn input() {
        assert_eq!(
            236539226447469,
            chronospatial_computer(include_str!("input"))
        );
    }
}
