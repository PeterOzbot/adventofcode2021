use std::fs;

enum CommandType {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn read_commands() -> Vec<CommandType> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut commands: Vec<CommandType> = vec![];

    for command_str in contents.split("\n") {
        let command_parts: Vec<&str> = command_str.split(" ").collect();
        let command_value = command_parts[1]
            .parse::<i32>()
            .expect("Something went wrong parsing measurement");

        let command = match command_parts[0] {
            "up" => CommandType::Up(command_value),
            "down" => CommandType::Down(command_value),
            "forward" => CommandType::Forward(command_value),
            _ => panic!("command not found"),
        };

        commands.push(command);
    }

    commands
}

fn part1() {
    let mut depth = 0;
    let mut horizontal = 0;
    for command in read_commands() {
        match command {
            CommandType::Down(value) => depth += value,
            CommandType::Up(value) => depth -= value,
            CommandType::Forward(value) => horizontal += value,
        }
    }

    println!(
        "Horizontal: {} Depth: {} Multiple: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn part2(){
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim= 0;

    for command in read_commands() {
        match command {
            CommandType::Down(value) => aim += value,
            CommandType::Up(value) => aim -= value,
            CommandType::Forward(value) => {
                horizontal += value;
                depth += value*aim;
            },
        }
    }

    println!(
        "Horizontal: {} Depth: {} Multiple: {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn main() {
    part1();
    part2();
}
