use crate::{get_garden_info, parse};

pub fn garden_groups(input: &str) -> usize {
    let garden = parse(input);

    get_garden_info(&garden)
        .iter()
        .map(|(a, p)| a.len() * p)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;

        let result = garden_groups(input);
        assert_eq!(140, result);
    }

    #[test]
    fn example_2() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

        let result = garden_groups(input);
        assert_eq!(772, result);
    }

    #[test]
    fn example_3() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

        let result = garden_groups(input);
        assert_eq!(1930, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = garden_groups(input);
        assert_eq!(1374934, result);
    }
}
