use std::collections::HashSet;

use crate::{get_garden_info, parse};

pub fn garden_groups(input: &str) -> usize {
    let garden = parse(input);

    let info = get_garden_info(&garden);

    info.iter()
        .map(|(region, _)| (region.len(), get_sides(region)))
        .map(|(a, s)| a * s)
        .sum()
}

fn get_sides(region: &HashSet<(usize, usize)>) -> usize {
    let mut corners = 0;

    for &(row, col) in region {
        for (d_row, d_col) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let e_row = extend_row(region, row, d_row, col).unwrap_or_default();
            let e_col = extend_col(region, row, col, d_col).unwrap_or_default();
            let e_diag = extend_diagonally(region, row, d_row, col, d_col).unwrap_or_default();

            if !e_row && !e_col {
                corners += 1;
            }

            if e_row && e_col && !e_diag {
                corners += 1;
            }
        }
    }
    corners
}

fn extend_row(
    region: &HashSet<(usize, usize)>,
    row: usize,
    d_row: isize,
    col: usize,
) -> Option<bool> {
    let point = (row.checked_add_signed(d_row)?, col);

    Some(region.contains(&point))
}

fn extend_col(
    region: &HashSet<(usize, usize)>,
    row: usize,
    col: usize,
    d_col: isize,
) -> Option<bool> {
    let point = (row, col.checked_add_signed(d_col)?);

    Some(region.contains(&point))
}

fn extend_diagonally(
    region: &HashSet<(usize, usize)>,
    row: usize,
    d_row: isize,
    col: usize,
    d_col: isize,
) -> Option<bool> {
    let point = (
        row.checked_add_signed(d_row)?,
        col.checked_add_signed(d_col)?,
    );

    Some(region.contains(&point))
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
        assert_eq!(80, result);
    }

    #[test]
    fn example_2() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;

        let result = garden_groups(input);
        assert_eq!(436, result);
    }

    #[test]
    fn example_3() {
        let input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

        let result = garden_groups(input);
        assert_eq!(236, result);
    }

    #[test]
    fn example_4() {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        let result = garden_groups(input);
        assert_eq!(368, result);
    }

    #[test]
    fn example_5() {
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
        assert_eq!(1206, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = garden_groups(input);
        assert_eq!(841078, result);
    }
}
