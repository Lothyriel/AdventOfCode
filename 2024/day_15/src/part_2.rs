use std::collections::{HashSet, VecDeque};

use crate::{new_pos_unchecked, parse_commands, Direction, Matrix, Movement, Point};

pub fn warehouse_woes(input: &str) -> usize {
    let (mut warehouse, commands) = parse(input);

    warehouse.process_commands(&commands);

    warehouse.gps_coordinates_count()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    BoxLeft,
    BoxRight,
    Wall,
    Robot,
    Empty,
}

impl Tile {
    fn box_halve(&self) -> Option<Half> {
        match self {
            Tile::BoxLeft => Some(Half::Left),
            Tile::BoxRight => Some(Half::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Warehouse {
    grid: Matrix<Tile>,
    robot: Point,
    explored: HashSet<Point>,
}

impl Warehouse {
    fn process_commands(&mut self, commands: &[Direction]) {
        for command in commands {
            self.process_command(command);
        }
    }

    pub fn gps_coordinates_count(&self) -> usize {
        self.grid
            .get_coordinates(|t| *t == Tile::BoxLeft)
            .map(|(x, y)| x * 100 + y)
            .sum()
    }

    pub fn process_command(&mut self, command: &Direction) -> Option<()> {
        self.explored.clear();

        match self.valid_movement(self.robot, command)? {
            Movement::Push(pushes) => self.push(pushes, command),
            Movement::Normal(_) => {}
        };

        let new = new_pos_unchecked(command, self.robot)?;

        self.grid.set(self.robot, Tile::Empty);
        self.grid.set(new, Tile::Robot);
        self.robot = new;

        Some(())
    }

    fn push(&mut self, pushes: VecDeque<Point>, direction: &Direction) {
        for (pos, tile) in self.get_new_positions(&pushes, direction) {
            self.grid.set(pos, tile);
        }
    }

    fn get_new_positions(
        &mut self,
        pushes: &VecDeque<Point>,
        direction: &Direction,
    ) -> VecDeque<(Point, Tile)> {
        let mut positions = VecDeque::new();

        for &pos in pushes.iter().rev() {
            let tile = *self.grid.get(pos).expect("Valid tile");

            if let (Tile::BoxLeft | Tile::BoxRight, Direction::Up | Direction::Down) =
                (tile, direction)
            {
                let empty = (self.get_other_half(pos).expect("Valid half"), Tile::Empty);
                positions.push_front(empty);
            };

            let new = new_pos_unchecked(direction, pos).expect("Valid pos");

            positions.push_back((new, tile));
        }

        positions
    }

    fn valid_movement(&mut self, from: Point, direction: &Direction) -> Option<Movement> {
        let new = new_pos_unchecked(direction, from)?;

        match self.grid.get(new)? {
            Tile::Wall => None,
            Tile::BoxLeft | Tile::BoxRight => self.valid_push(new, direction).map(Movement::Push),
            Tile::Empty => Some(Movement::Normal(new)),
            Tile::Robot => panic!("Shouldn't happen..."),
        }
    }

    fn valid_push(&mut self, from: Point, direction: &Direction) -> Option<VecDeque<Point>> {
        let mut to_push = VecDeque::new();
        let mut to_explore = VecDeque::from([from]);

        while let Some(from) = to_explore.pop_front() {
            if !self.explored.insert(from) {
                continue;
            }

            to_explore.push_front(self.get_other_half(from)?);
            to_push.push_front(from);

            let movement = self.valid_movement(from, direction);

            match movement? {
                Movement::Push(mut pushes) => {
                    while let Some(n) = pushes.pop_front() {
                        to_push.push_back(n);
                    }
                }
                Movement::Normal(_) => {}
            };
        }

        Some(to_push)
    }

    pub fn debug_grid(&self) -> String {
        let mut s = String::new();

        for x in 0..self.grid.rows {
            for y in 0..self.grid.cols {
                let ch = match self.grid.get((x, y)).expect("Valid") {
                    Tile::Wall => '#',
                    Tile::BoxLeft => '[',
                    Tile::BoxRight => ']',
                    Tile::Empty => '.',
                    Tile::Robot => '@',
                };

                s.push_str(&format!("{}", ch));
            }
            s.push('\n');
        }

        s
    }

    fn get_other_half(&self, from: Point) -> Option<Point> {
        let direction = match self.grid.get(from)?.box_halve()? {
            Half::Left => Direction::Right,
            Half::Right => Direction::Left,
        };

        let other_pos = new_pos_unchecked(&direction, from)?;

        self.grid.get(other_pos)?.box_halve()?;

        Some(other_pos)
    }
}

pub fn parse(input: &str) -> (Warehouse, Vec<Direction>) {
    let mut parts = input.split("\n\n");

    let warehouse = parse_warehouse(&scale_up(parts.next().expect("Grid")));

    let commands = parse_commands(parts.next().expect("Commands"));

    (warehouse, commands)
}

fn parse_warehouse(input: &str) -> Warehouse {
    let size = input.lines().next().expect("First").len();

    let inner = input
        .lines()
        .flat_map(|l| {
            l.bytes().map(|b| match b {
                b'#' => Tile::Wall,
                b'@' => Tile::Robot,
                b'.' => Tile::Empty,
                b'[' => Tile::BoxLeft,
                b']' => Tile::BoxRight,
                _ => panic!("Invalid tile"),
            })
        })
        .collect();

    let grid = Matrix {
        inner,
        rows: size / 2,
        cols: size,
    };

    let robot = grid
        .get_coordinates(|t| *t == Tile::Robot)
        .next()
        .expect("Robot");

    Warehouse {
        robot,
        grid,
        explored: HashSet::new(),
    }
}

fn scale_up(grid: &str) -> String {
    let mut s = String::new();
    let size = grid.lines().next().expect("First").len();

    for (i, line) in grid.lines().enumerate() {
        for row in line.bytes() {
            let a = match row {
                b'#' => "##",
                b'@' => "@.",
                b'.' => "..",
                b'O' => "[]",
                _ => panic!("Invalid tile"),
            };

            s.push_str(a);
        }

        if i != size - 1 {
            s.push('\n')
        }
    }

    s
}

#[derive(Clone, Copy)]
enum Half {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_up_test() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########"#;

        let expected = r#"####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################"#;

        assert_eq!(expected, scale_up(input));
    }

    #[test]
    fn smaller_example() {
        let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;

        assert_eq!(618, warehouse_woes(input));
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

        assert_eq!(9021, warehouse_woes(input));
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        assert_eq!(1481392, warehouse_woes(input));
    }
}
