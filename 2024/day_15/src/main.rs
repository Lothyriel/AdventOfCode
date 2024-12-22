fn main() {
    let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    part_2(input);
}

pub fn part_1(input: &str) {
    let (mut warehouse, commands) = day_15::part_1::parse(input);

    for command in &commands {
        clear_console();
        println!("{}", warehouse.debug_grid());
        warehouse.process_command(command);
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    println!("Result {}", warehouse.gps_coordinates_count())
}

pub fn part_2(input: &str) {
    let (mut warehouse, commands) = day_15::part_2::parse(input);

    println!("{}", warehouse.debug_grid());
    std::io::stdin().read_line(&mut String::new()).unwrap();
    for (i, command) in commands.iter().enumerate() {
        clear_console();
        println!("Command {i} {command:?}\n");
        warehouse.process_command(command);
        println!("{}", warehouse.debug_grid());
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    println!("Result {}", warehouse.gps_coordinates_count())
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .expect("Failed to clear the console");
    } else {
        print!("\x1B[2J\x1B[1;1H");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}
