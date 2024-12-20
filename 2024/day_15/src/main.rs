fn main() {
    let input = include_str!("input");
    part_1(input);
}

fn part_1(input: &str) {
    let (mut warehouse, commands) = day_15::part_1::parse(input);

    for command in &commands {
        clear_console();
        println!("{}", warehouse.debug_grid());
        println!("{command:?}");
        warehouse.process_command(command);
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
