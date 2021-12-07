use std::fs;

fn part1() {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut measurement_increased = 0;
    let mut previous_measurement: Option<i32> = None;

    for measurement_str in contents.split("\n") {
        let measurement = measurement_str
            .parse::<i32>()
            .expect("Something went wrong parsing measurement");

        if let Some(previous_value) = previous_measurement {
            if measurement > previous_value {
                measurement_increased += 1;
            }
        }

        previous_measurement = Some(measurement);
    }
    println!("Measurement increased: {}", measurement_increased);
}

fn part2() {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut measurements = vec![];

    for measurement_str in contents.split("\n") {
        let measurement = measurement_str
            .parse::<i32>()
            .expect("Something went wrong parsing measurement");
        measurements.push(measurement);
    }

    let mut measurement_increased = 0;

    for index in 0..measurements.len() - 3 {
        let current_window =
            measurements[index] + measurements[index + 1] + measurements[index + 2];
        let next_window =
            measurements[index + 1] + measurements[index + 2] + measurements[index + 3];
        if next_window > current_window {
            measurement_increased += 1;
        }
    }
    println!("Measurement increased: {}", measurement_increased);
}

fn main() {
    part1();
    part2();
}
