pub fn code_chronicle(input: &str) -> usize {
    let schematics = parse(input);

    let (keys, locks) = sort(schematics);

    get_overlap_count(&keys, &locks)
}

fn get_overlap_count(keys: &[Schematic], locks: &[Schematic]) -> usize {
    let mut count = 0;

    for key in keys {
        for lock in locks {
            if overlap(key, lock) {
                count += 1;
            }
        }
    }

    count
}

fn overlap(key: &Schematic, lock: &Schematic) -> bool {
    for x in 0..7 {
        for y in 0..5 {
            if key.pattern[x][y] == Row::Filled && lock.pattern[x][y] == Row::Filled {
                return false;
            }
        }
    }

    true
}

fn sort(schematics: Vec<Schematic>) -> (Vec<Schematic>, Vec<Schematic>) {
    let mut keys = vec![];
    let mut locks = vec![];

    for s in schematics {
        if s.is_key() {
            keys.push(s);
        } else if s.is_lock() {
            locks.push(s);
        } else {
            panic!("Neither")
        }
    }

    (keys, locks)
}

#[derive(Debug)]
struct Schematic {
    pattern: Vec<Vec<Row>>,
}
impl Schematic {
    fn is_key(&self) -> bool {
        self.pattern[0].iter().all(|r| *r == Row::Empty)
            && self.pattern[6].iter().all(|r| *r == Row::Filled)
    }

    fn is_lock(&self) -> bool {
        self.pattern[0].iter().all(|r| *r == Row::Filled)
            && self.pattern[6].iter().all(|r| *r == Row::Empty)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Row {
    Filled,
    Empty,
}

fn parse(input: &str) -> Vec<Schematic> {
    input.split("\n\n").map(parse_schematic).collect()
}

fn parse_schematic(input: &str) -> Schematic {
    let pattern = input.lines().map(parse_line).collect();

    Schematic { pattern }
}

fn parse_line(input: &str) -> Vec<Row> {
    input
        .bytes()
        .map(|b| match b {
            b'#' => Row::Filled,
            b'.' => Row::Empty,
            _ => panic!("Invalid value"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

        let result = code_chronicle(input);
        assert_eq!(3, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = code_chronicle(input);
        assert_eq!(3155, result);
    }
}
