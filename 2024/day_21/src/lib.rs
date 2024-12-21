use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
};

pub mod part_1;
pub mod part_2;

fn get_complexity(code: &Code, directional_robots_count: usize) -> usize {
    let seq = shortest_sequence(code, directional_robots_count);
    let code_value = numeric_value(code);

    seq * code_value
}

fn shortest_sequence(code: &[NumericKey], move_robots: usize) -> usize {
    let mut memo = HashMap::new();

    let init = (NumericKey::Activate, 0);

    let (_, total) = code.iter().fold(init, |(curr, total), next| {
        let result = dijkstra(move_robots + 1, curr, *next, get_num_neighbours, &mut memo);

        (*next, total + result + 1)
    });

    total
}

type State = (usize, DirectionalKey, DirectionalKey);

type Code = Vec<NumericKey>;

fn dijkstra<T: Eq + Hash + Copy>(
    depth: usize,
    start: T,
    end: T,
    get_neighbours: fn(T) -> Vec<(T, DirectionalKey)>,
    memo: &mut HashMap<State, usize>,
) -> usize {
    if depth == 0 {
        return 0;
    }

    let mut visited = HashSet::new();
    let mut to_visit = BTreeMap::new();

    to_visit.insert(0, vec![(start, DirectionalKey::Activate)]);

    let target = (end, DirectionalKey::Activate);

    while let Some((current_cost, paths)) = to_visit.pop_first() {
        for (from, to) in paths {
            if !visited.insert((from, to)) {
                continue;
            }

            if (from, to) == target {
                return current_cost;
            }

            let neighbors = if from == end {
                vec![(from, DirectionalKey::Activate)]
            } else {
                get_neighbours(from)
            };

            for n in neighbors {
                if !visited.contains(&n) {
                    let k = (depth - 1, to, n.1);

                    let new_cost = if let Some(cost) = memo.get(&k) {
                        *cost
                    } else {
                        let cost = match n.1 {
                            DirectionalKey::Activate => {
                                dijkstra(depth - 1, to, n.1, get_dir_neighbours, memo)
                            }
                            _ => 1 + dijkstra(depth - 1, to, n.1, get_dir_neighbours, memo),
                        };

                        memo.insert(k, cost);

                        cost
                    };

                    to_visit
                        .entry(current_cost + new_cost)
                        .or_insert(vec![])
                        .push(n);
                }
            }
        }
    }

    panic!("No valid path found")
}

fn get_num_neighbours(key: NumericKey) -> Vec<(NumericKey, DirectionalKey)> {
    match key {
        NumericKey::Zero => vec![
            (NumericKey::Activate, DirectionalKey::Right),
            (NumericKey::Two, DirectionalKey::Up),
        ],
        NumericKey::One => vec![
            (NumericKey::Four, DirectionalKey::Up),
            (NumericKey::Two, DirectionalKey::Right),
        ],
        NumericKey::Two => vec![
            (NumericKey::One, DirectionalKey::Left),
            (NumericKey::Zero, DirectionalKey::Down),
            (NumericKey::Three, DirectionalKey::Right),
            (NumericKey::Five, DirectionalKey::Up),
        ],
        NumericKey::Three => vec![
            (NumericKey::Two, DirectionalKey::Left),
            (NumericKey::Activate, DirectionalKey::Down),
            (NumericKey::Six, DirectionalKey::Up),
        ],
        NumericKey::Four => vec![
            (NumericKey::One, DirectionalKey::Down),
            (NumericKey::Five, DirectionalKey::Right),
            (NumericKey::Seven, DirectionalKey::Up),
        ],
        NumericKey::Five => vec![
            (NumericKey::Four, DirectionalKey::Left),
            (NumericKey::Two, DirectionalKey::Down),
            (NumericKey::Six, DirectionalKey::Right),
            (NumericKey::Eight, DirectionalKey::Up),
        ],
        NumericKey::Six => vec![
            (NumericKey::Five, DirectionalKey::Left),
            (NumericKey::Three, DirectionalKey::Down),
            (NumericKey::Nine, DirectionalKey::Up),
        ],
        NumericKey::Seven => vec![
            (NumericKey::Four, DirectionalKey::Down),
            (NumericKey::Eight, DirectionalKey::Right),
        ],
        NumericKey::Eight => vec![
            (NumericKey::Seven, DirectionalKey::Left),
            (NumericKey::Five, DirectionalKey::Down),
            (NumericKey::Nine, DirectionalKey::Right),
        ],
        NumericKey::Nine => vec![
            (NumericKey::Eight, DirectionalKey::Left),
            (NumericKey::Six, DirectionalKey::Down),
        ],
        NumericKey::Activate => vec![
            (NumericKey::Zero, DirectionalKey::Left),
            (NumericKey::Three, DirectionalKey::Up),
        ],
    }
}

fn get_dir_neighbours(key: DirectionalKey) -> Vec<(DirectionalKey, DirectionalKey)> {
    match key {
        DirectionalKey::Activate => vec![
            (DirectionalKey::Right, DirectionalKey::Down),
            (DirectionalKey::Up, DirectionalKey::Left),
        ],
        DirectionalKey::Up => vec![
            (DirectionalKey::Down, DirectionalKey::Down),
            (DirectionalKey::Activate, DirectionalKey::Right),
        ],
        DirectionalKey::Down => vec![
            (DirectionalKey::Up, DirectionalKey::Up),
            (DirectionalKey::Left, DirectionalKey::Left),
            (DirectionalKey::Right, DirectionalKey::Right),
        ],
        DirectionalKey::Left => vec![(DirectionalKey::Down, DirectionalKey::Right)],
        DirectionalKey::Right => vec![
            (DirectionalKey::Down, DirectionalKey::Left),
            (DirectionalKey::Activate, DirectionalKey::Up),
        ],
    }
}

fn get_codes_complexity(codes: &[Code], directional_robots_count: usize) -> usize {
    codes
        .iter()
        .map(|code| get_complexity(code, directional_robots_count))
        .sum()
}

fn numeric_value(code: &Code) -> usize {
    code.iter()
        .rev()
        .skip(1)
        .enumerate()
        .map(|(i, k)| {
            let base = match k {
                NumericKey::Activate => 0,
                n => *n as usize,
            };

            base * 10usize.pow(i as u32)
        })
        .sum()
}

fn parse(input: &str) -> Vec<Code> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'0' => NumericKey::Zero,
                    b'1' => NumericKey::One,
                    b'2' => NumericKey::Two,
                    b'3' => NumericKey::Three,
                    b'4' => NumericKey::Four,
                    b'5' => NumericKey::Five,
                    b'6' => NumericKey::Six,
                    b'7' => NumericKey::Seven,
                    b'8' => NumericKey::Eight,
                    b'9' => NumericKey::Nine,
                    b'A' => NumericKey::Activate,
                    _ => panic!("Invalid code"),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum NumericKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DirectionalKey {
    Up,
    Down,
    Left,
    Right,
    Activate,
}
