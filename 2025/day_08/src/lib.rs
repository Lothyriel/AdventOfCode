use std::collections::{HashMap, VecDeque};

pub mod part_1;
pub mod part_2;

fn parse(input: &str) -> impl Iterator<Item = Box> {
    input.lines().map(|l| {
        let mut parts = l.split(',');

        let mut m = || parts.next().unwrap().parse().unwrap();

        let x = m();
        let y = m();
        let z = m();

        Box { x, y, z }
    })
}

fn get_unique_pairs(boxes: &[Box]) -> Vec<(isize, usize, usize)> {
    let n = boxes.len();
    let mut pairs = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = boxes[i].dist_sq(&boxes[j]);
            pairs.push((d, i, j));
        }
    }

    pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    pairs
}

fn component_sizes(graph: &HashMap<usize, Vec<usize>>, n: usize) -> Vec<usize> {
    let mut visited = vec![false; n];
    let mut sizes = Vec::new();

    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut size = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited[start] = true;

        while let Some(u) = queue.pop_front() {
            size += 1;

            if let Some(neighbor) = graph.get(&u) {
                for &v in neighbor {
                    if !visited[v] {
                        visited[v] = true;
                        queue.push_back(v);
                    }
                }
            }
        }

        sizes.push(size);
    }

    sizes
}

struct Box {
    x: isize,
    y: isize,
    z: isize,
}

impl Box {
    fn dist_sq(&self, b: &Box) -> isize {
        let dx = self.x - b.x;
        let dy = self.y - b.y;
        let dz = self.z - b.z;

        dx * dx + dy * dy + dz * dz
    }
}
