use std::collections::{BinaryHeap, HashSet};

use crate::{parse, MazeRunner, Point, State, Tile};

pub fn reindeer_maze(input: &str) -> usize {
    let runner = parse(input);

    let finish = runner
        .maze
        .get_coordinates(|x| *x == Tile::Finish)
        .next()
        .expect("Finish");

    runner.a_star(finish).expect("Path")
}

impl MazeRunner {
    fn a_star(&self, finish: (usize, usize)) -> Option<usize> {
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();

        to_visit.push(State {
            point: Point {
                coordinates: self.position,
                direction: self.direction,
            },
            cost: 0,
        });

        while let Some(state) = to_visit.pop() {
            let (x, y) = state.point.coordinates;

            if state.point.coordinates == finish {
                return Some(state.cost);
            }

            let visited_state = (x, y, state.point.direction as u8);

            if visited.contains(&visited_state) {
                continue;
            }

            visited.insert(visited_state);

            if let Some(new_coordinates) = self.get_forward_coordinates(&state) {
                to_visit.push(State {
                    point: state.point.new_coordinates(new_coordinates),
                    cost: state.cost + 1,
                });
            }

            let rotations = [
                state.point.direction.rotate_left(),
                state.point.direction.rotate_right(),
            ];

            for direction in rotations {
                to_visit.push(State {
                    point: state.point.new_direction(direction),
                    cost: state.cost + 1000,
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

        assert_eq!(7036, reindeer_maze(input));
    }

    #[test]
    fn example_2() {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

        assert_eq!(11048, reindeer_maze(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        assert_eq!(101492, reindeer_maze(input));
    }
}
