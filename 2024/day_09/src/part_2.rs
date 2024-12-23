use crate::{expand_map, parse};

pub fn disk_fragmenter(input: &str) -> usize {
    let disk_map = parse(input);

    let mut memory = expand_map(&disk_map);

    defrag(&mut memory);

    memory
        .iter()
        .enumerate()
        .flat_map(|(i, &m)| Some(i * m?))
        .sum()
}

fn defrag(memory: &mut [Option<usize>]) {
    let mut biggest_id = memory.last().expect("Last").expect("Id");

    loop {
        let (f_start, f_end) = get_file_pos(memory, biggest_id);
        let f_len = f_start.abs_diff(f_end) + 1;

        if let Some((e_start, _)) = get_empty_pos(memory, f_len) {
            if e_start < f_start {
                (0..f_len).for_each(|i| {
                    memory[i + f_start] = None;
                });

                (0..f_len).for_each(|i| {
                    memory[i + e_start] = Some(biggest_id);
                });
            }
        }

        if biggest_id == 0 {
            return;
        }
        biggest_id -= 1;
    }
}

fn get_empty_pos(memory: &mut [Option<usize>], min_size: usize) -> Option<(usize, usize)> {
    let mut pos = None;

    for (i, m) in memory.iter().enumerate() {
        match (m, pos) {
            (None, None) => pos = Some((i, i)),
            (None, Some((start, _))) => pos = Some((start, i)),
            (Some(_), None) => continue,
            (Some(_), Some(p)) => {
                if p.0.abs_diff(p.1) + 1 >= min_size {
                    return pos;
                } else {
                    pos = None
                }
            }
        };
    }

    pos
}

fn get_file_pos(memory: &mut [Option<usize>], file_id: usize) -> (usize, usize) {
    let mut pos = None;

    let iter = memory
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(i, &m)| Some((i, m?)));

    for (i, m) in iter {
        match (m == file_id, pos) {
            (true, None) => pos = Some((i, i)),
            (true, Some((_, end))) => pos = Some((i, end)),
            (false, None) => continue,
            (false, Some(pos)) => return pos,
        };
    }

    pos.expect("Should've found it")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2333133121414131402";
        let result = disk_fragmenter(input);
        assert_eq!(2858, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = disk_fragmenter(input);
        assert_eq!(6307279963620, result);
    }
}
