use crate::{parse, Matrix};

pub fn ceres_search(input: &str) -> usize {
    let matrix = parse(input);

    let scan_patterns = [
        Matrix::horizontal_forwards,
        Matrix::horizontal_backwards,
        Matrix::vertical_upwards,
        Matrix::vertical_downwards,
        Matrix::diagonal_ne,
        Matrix::diagonal_nw,
        Matrix::diagonal_se,
        Matrix::diagonal_sw,
    ];

    matrix.xmas_count(&scan_patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let matrix = parse(input);

        assert_eq!(3, matrix.xmas_count(&[Matrix::horizontal_forwards]));
        assert_eq!(2, matrix.xmas_count(&[Matrix::horizontal_backwards]));
        assert_eq!(1, matrix.xmas_count(&[Matrix::vertical_downwards]));
        assert_eq!(2, matrix.xmas_count(&[Matrix::vertical_upwards]));
        assert_eq!(4, matrix.xmas_count(&[Matrix::diagonal_ne]));
        assert_eq!(4, matrix.xmas_count(&[Matrix::diagonal_nw]));
        assert_eq!(1, matrix.xmas_count(&[Matrix::diagonal_se]));
        assert_eq!(1, matrix.xmas_count(&[Matrix::diagonal_sw]));

        assert_eq!(18, ceres_search(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");

        assert_eq!(2462, ceres_search(input));
    }
}
