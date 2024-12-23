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
    loop {
        let empty_slot_idx = memory
            .iter()
            .enumerate()
            .find_map(|(i, &m)| if m.is_none() { Some(i) } else { None })
            .expect("Empty slot");

        let block_idx = memory
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, m)| if m.is_some() { Some(i) } else { None })
            .expect("Last block");

        if empty_slot_idx > block_idx {
            return;
        }

        memory.swap(empty_slot_idx, block_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2333133121414131402";
        let result = disk_fragmenter(input);
        assert_eq!(1928, result);
    }

    #[test]
    fn input() {
        let input = include_str!("input");
        let result = disk_fragmenter(input);
        assert_eq!(6291146824486, result);
    }
}
