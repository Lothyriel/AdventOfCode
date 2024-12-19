use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::{parse, MazeRunner, Point, State, Tile};

pub fn reindeer_maze(input: &str) -> usize {
    let runner = parse(input);

    let finish = runner
        .maze
        .get_coordinates(|x| *x == Tile::Finish)
        .next()
        .expect("Finish");

    let best_paths = runner.dijkstra(finish);

    let best_tiles: HashSet<_> = best_paths.iter().flatten().map(|x| x.coordinates).collect();

    best_tiles.len()
}

impl MazeRunner {
    fn dijkstra(&self, finish: (usize, usize)) -> Vec<Vec<Point>> {
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut to_visit = BinaryHeap::new();
        let mut min_cost = None;

        let start = Point {
            coordinates: self.position,
            direction: self.direction,
        };

        to_visit.push(State {
            cost: 0,
            point: start,
        });

        distances.insert(start, 0);
        previous.insert(start, HashSet::new());

        while let Some(current) = to_visit.pop() {
            if let Some(best_cost) = min_cost {
                if current.cost > best_cost {
                    continue;
                }
            }

            if current.point.coordinates == finish {
                min_cost = Some(current.cost);
                continue;
            }

            if let Some(new_coordinates) = self.get_forward_coordinates(&current) {
                let next = State {
                    point: current.point.new_coordinates(new_coordinates),
                    cost: current.cost + 1,
                };

                process_next_state(
                    next,
                    current.point,
                    &mut distances,
                    &mut previous,
                    &mut to_visit,
                );
            }

            let rotations = [
                current.point.direction.rotate_right(),
                current.point.direction.rotate_left(),
            ];

            for direction in rotations {
                let next = State {
                    point: current.point.new_direction(direction),
                    cost: current.cost + 1000,
                };

                process_next_state(
                    next,
                    current.point,
                    &mut distances,
                    &mut previous,
                    &mut to_visit,
                );
            }
        }

        distances
            .keys()
            .filter(|p| p.coordinates == finish)
            .map(|p| reconstruct_paths(&previous, *p).into())
            .collect()
    }
}

fn reconstruct_paths(previous: &HashMap<Point, HashSet<Point>>, current: Point) -> VecDeque<Point> {
    let mut current_path = VecDeque::new();
    current_path.push_front(current);

    for prev in &previous[&current] {
        current_path.push_front(*prev);

        for tile in reconstruct_paths(previous, *prev) {
            current_path.push_front(tile);
        }
    }

    current_path
}

fn process_next_state(
    next: State,
    current: Point,
    distances: &mut HashMap<Point, usize>,
    previous: &mut HashMap<Point, HashSet<Point>>,
    to_visit: &mut BinaryHeap<State>,
) {
    if !distances.contains_key(&next.point) || next.cost <= distances[&next.point] {
        if Some(&next.cost) == distances.get(&next.point) {
            previous.entry(next.point).or_default().insert(current);
        } else {
            distances.insert(next.point, next.cost);
            previous.insert(next.point, HashSet::from([current]));
        }

        to_visit.push(next);
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

        assert_eq!(45, reindeer_maze(input));
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

        assert_eq!(64, reindeer_maze(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        assert_eq!(543, reindeer_maze(input));
    }
}
