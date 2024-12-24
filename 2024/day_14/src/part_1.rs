use crate::{parse, simulate, Point, Vec2};

pub fn restroom_redoubt(input: &str, grid: Point) -> usize {
    let mut robots = parse(input);

    simulate(&mut robots, grid, 100);

    count_robots(&robots, grid)
}

fn count_robots(robots: &[(Point, Vec2)], grid: Point) -> usize {
    let (rows, cols) = grid;
    let center_row = rows / 2;
    let center_col = cols / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for &((row, col), _) in robots {
        if row == center_row || col == center_col {
            continue;
        }

        if row < center_row && col < center_col {
            top_left += 1;
        } else if row < center_row && col >= center_col {
            top_right += 1;
        } else if row >= center_row && col < center_col {
            bottom_left += 1;
        } else {
            bottom_right += 1;
        }
    }

    top_left * top_right * bottom_left * bottom_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

        let result = restroom_redoubt(input, (11, 7));
        assert_eq!(12, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = restroom_redoubt(input, (101, 103));
        assert_eq!(225810288, result);
    }
}
