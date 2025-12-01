pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> impl Iterator<Item = Rotation> {
    input.lines().map(|l| {
        let mut bytes = l.bytes();

        let direction = bytes.next().unwrap();

        let remaining = bytes.collect();

        let distance = String::from_utf8(remaining).unwrap().parse().unwrap();

        match direction {
            b'L' => Rotation::Left(distance),
            b'R' => Rotation::Right(distance),
            _ => panic!("invalid direction"),
        }
    })
}

struct Dial {
    pos: isize,
}

impl Dial {
    fn new() -> Self {
        Self { pos: 50 }
    }

    fn apply_rotation(&mut self, rotation: &Rotation) {
        let change = match rotation {
            Rotation::Left(d) => self.pos - d,
            Rotation::Right(d) => self.pos + d,
        };

        self.pos = change % 100
    }

    // this is not good :(
    fn apply_rotation_steps(&mut self, rotation: &Rotation) -> isize {
        let (step, distance) = match rotation {
            Rotation::Left(d) => (-1, d),
            Rotation::Right(d) => (1, d),
        };

        let mut count = 0;
        for _ in 0..*distance {
            self.pos = (self.pos + step) % 100;

            if self.pos == 0 {
                count += 1;
            }
        }

        count
    }
}

enum Rotation {
    Left(isize),
    Right(isize),
}
