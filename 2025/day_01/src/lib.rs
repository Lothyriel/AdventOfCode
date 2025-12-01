pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> impl Iterator<Item = Rotation> {
    input.lines().map(|l| {
        let mut bytes = l.bytes();

        let dir = bytes.next().unwrap();

        let rem = bytes.collect();

        let dist = String::from_utf8(rem).unwrap().parse().unwrap();

        let dir = match dir {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("invalid direction"),
        };

        Rotation { dir, dist }
    })
}

struct Dial {
    pos: isize,
    size: isize,
}

impl Dial {
    fn new() -> Self {
        Self { pos: 50, size: 100 }
    }

    fn apply(&mut self, rotation: &Rotation) {
        let change = match rotation.dir {
            Direction::Left => self.pos - rotation.dist,
            Direction::Right => self.pos + rotation.dist,
        };

        self.pos = change % self.size
    }

    fn apply_get_wrap_count(&mut self, rotation: &Rotation) -> isize {
        let rem = rotation.dist % self.size;
        let count = rotation.dist / self.size;

        match rotation.dir {
            Direction::Left => {
                let count = if self.pos + rem >= self.size {
                    count + 1
                } else {
                    count
                };

                self.pos = (self.pos + rem) % self.size;

                count
            }
            Direction::Right => {
                let wraps = if self.pos > 0 && self.pos - rem <= 0 {
                    count + 1
                } else {
                    count
                };

                self.pos = ((self.pos - rem) % self.size + self.size) % self.size;

                wraps
            }
        }
    }
}

enum Direction {
    Left,
    Right,
}

struct Rotation {
    dist: isize,
    dir: Direction,
}
