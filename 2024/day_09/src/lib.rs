pub mod part_1;
pub mod part_2;

fn expand_map(disk_map: &[u8]) -> Vec<Option<usize>> {
    let mut memory = vec![];

    for (id, i) in (0..disk_map.len()).step_by(2).enumerate() {
        let block_size = disk_map[i];

        for _ in 0..block_size {
            memory.push(Some(id));
        }

        let Some(&empty_block_size) = disk_map.get(i + 1) else {
            continue;
        };

        for _ in 0..empty_block_size {
            memory.push(None);
        }
    }

    memory
}

fn parse(input: &str) -> Vec<u8> {
    input.trim().bytes().map(|b| b - 48).collect()
}
