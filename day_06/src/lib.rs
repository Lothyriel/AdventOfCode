pub mod part_1;
pub mod part_2;

struct Record {
    time: usize,
    distance: usize,
}

fn get_winning_strategies(record: Record) -> impl Iterator<Item = (usize, usize)> {
    (0..=record.time)
        .map(move |pressed_t| (pressed_t, (record.time - pressed_t) * pressed_t))
        .filter(move |(_, distance)| *distance > record.distance)
}
