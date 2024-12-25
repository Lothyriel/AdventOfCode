use std::collections::{HashSet, VecDeque};

pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

type GardenInfo = (HashSet<(usize, usize)>, usize);

fn get_garden_info(garden: &[Vec<char>]) -> Vec<GardenInfo> {
    let rows = garden.len();
    let cols = garden[0].len();
    let mut visited = HashSet::new();
    let mut info = vec![];

    for y in 0..rows {
        for x in 0..cols {
            if visited.contains(&(y, x)) {
                continue;
            }

            let region = explore_region(garden, (y, x), &mut visited);
            info.push(region);
        }
    }

    info
}

fn explore_region(
    garden: &[Vec<char>],
    current: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> GardenInfo {
    let height = garden.len() as isize;
    let width = garden[0].len() as isize;
    let plant_type = garden[current.0][current.1];
    let mut region = HashSet::from([current]);

    let mut to_explore = VecDeque::from([current]);
    let mut perimeter = 0;

    visited.insert(current);

    while let Some((y, x)) = to_explore.pop_front() {
        for (dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny >= 0 && ny < height && nx >= 0 && nx < width {
                let next = (ny as usize, nx as usize);

                if garden[next.0][next.1] == plant_type {
                    if !visited.contains(&next) {
                        region.insert(next);
                        visited.insert(next);
                        to_explore.push_back(next);
                    }
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (region, perimeter)
}
