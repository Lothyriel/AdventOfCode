use std::collections::VecDeque;

use crate::{new_pos_unchecked, parse_commands, Direction, Matrix, Movement, Point};

pub fn warehouse_woes(input: &str) -> usize {
    let (mut warehouse, commands) = parse(input);

    warehouse.process_commands(&commands);

    warehouse.gps_coordinates_count()
}

pub fn parse(input: &str) -> (Warehouse, Vec<Direction>) {
    let mut parts = input.split("\n\n");

    let size = input.lines().next().expect("First line").bytes().len();

    let inner = parts
        .next()
        .expect("Grid")
        .lines()
        .flat_map(|l| {
            l.bytes().map(|b| match b {
                b'#' => Tile::Wall,
                b'@' => Tile::Robot,
                b'.' => Tile::Empty,
                b'O' => Tile::Box,
                _ => panic!("Invalid tile"),
            })
        })
        .collect();

    let grid = Matrix {
        inner,
        cols: size,
        rows: size,
    };

    let robot = grid
        .get_coordinates(|t| *t == Tile::Robot)
        .next()
        .expect("Robot");

    let commands = parse_commands(parts.next().expect("Commands"));

    (Warehouse { robot, grid }, commands)
}

pub struct Warehouse {
    grid: Matrix<Tile>,
    robot: Point,
}

impl Warehouse {
    fn process_commands(&mut self, commands: &[Direction]) {
        for command in commands {
            self.process_command(command);
        }
    }

    pub fn gps_coordinates_count(&self) -> usize {
        self.grid
            .get_coordinates(|t| *t == Tile::Box)
            .map(|(x, y)| x * 100 + y)
            .sum()
    }

    pub fn process_command(&mut self, command: &Direction) -> Option<()> {
        let new = match self.valid_movement(self.robot, command)? {
            Movement::Push(pushes) => self.push(pushes, command),
            Movement::Normal(new) => new,
        };

        self.grid.set(self.robot, Tile::Empty);
        self.grid.set(new, Tile::Robot);
        self.robot = new;

        Some(())
    }

    fn push(&mut self, pushes: VecDeque<Point>, direction: &Direction) -> Point {
        for &push in pushes.iter() {
            self.grid.set(push, Tile::Empty);

            let push = new_pos_unchecked(direction, push).expect("Valid pos");

            self.grid.set(push, Tile::Box);
        }

        *pushes.iter().last().expect("First push")
    }

    fn valid_movement(&self, from: Point, direction: &Direction) -> Option<Movement> {
        let new = new_pos_unchecked(direction, from)?;

        match self.grid.get(new)? {
            Tile::Wall => None,
            Tile::Box => self.valid_push(new, direction).map(Movement::Push),
            Tile::Empty => Some(Movement::Normal(new)),
            Tile::Robot => panic!("Shouldn't happen..."),
        }
    }

    fn valid_push(&self, from: Point, direction: &Direction) -> Option<VecDeque<Point>> {
        let mut pushes = VecDeque::from([from]);

        match self.valid_movement(from, direction)? {
            Movement::Push(mut p) => {
                while let Some(p) = p.pop_back() {
                    pushes.push_front(p);
                }
                Some(pushes)
            }
            Movement::Normal(_) => Some(pushes),
        }
    }

    pub fn debug_grid(&self) -> String {
        let mut s = String::new();

        for x in 0..self.grid.rows {
            for y in 0..self.grid.cols {
                let ch = match self.grid.get((x, y)).expect("Valid") {
                    Tile::Wall => '#',
                    Tile::Box => 'O',
                    Tile::Empty => '.',
                    Tile::Robot => '@',
                };

                s.push(ch);
            }
            s.push('\n');
        }

        s
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    Empty,
    Robot,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_example() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

        assert_eq!(2028, warehouse_woes(input));
    }

    #[test]
    fn larger_example() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

        assert_eq!(10092, warehouse_woes(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        assert_eq!(1463715, warehouse_woes(input));
    }
}
