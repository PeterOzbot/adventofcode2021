use std::fs;

fn read_data() -> Vec<i32> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|number| number.parse::<i32>().unwrap())
        .collect()
}

fn part1() {
    let crab_positions = read_data();
    let mut minimum_fuel = i32::MAX;
    let mut minimum_position = 0;

    for position in &crab_positions {
        let mut fuel = 0;
        for comparing_position in &crab_positions {
            fuel += (position - comparing_position).abs();
        }
        if fuel < minimum_fuel {
            minimum_fuel = fuel;
            minimum_position = *position;
        }
    }

    println!("Position: {}, Fuel: {}", minimum_position, minimum_fuel)
}

fn part2() {
    let crab_positions = read_data();
    let mut minimum_fuel = i32::MAX;
    let mut minimum_position = 0;

    for position in 1..1500 {
        // just some arbitrary number, increase/decrease if needed
        let mut fuel = 0;
        for comparing_position in &crab_positions {
            fuel += calculate_fuel((position - comparing_position).abs());
        }
        if fuel < minimum_fuel {
            minimum_fuel = fuel;
            minimum_position = position;
        }
    }

    println!("Position: {}, Fuel: {}", minimum_position, minimum_fuel)
}

fn calculate_fuel(distance: i32) -> i32 {
    //Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.
    (distance * (distance + 1)) / 2
}

fn main() {
    part1();
    part2();
}
