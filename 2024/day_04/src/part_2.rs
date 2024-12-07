use crate::parse;

pub fn ceres_search(input: &str) -> usize {
    let matrix = parse(input);

    matrix.x_mas_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;

        assert_eq!(9, ceres_search(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(1877, ceres_search(input));
    }
}
