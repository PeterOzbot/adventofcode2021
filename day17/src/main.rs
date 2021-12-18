use std::fs;

#[derive(Debug)]
pub struct TargetArea {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}
fn read_data() -> TargetArea {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let data: Vec<Vec<i32>> = contents
        .replace("target area: ", "")
        .replace("x=", "")
        .replace("y=", "")
        .split(", ")
        .map(|range| {
            range
                .split("..")
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    TargetArea {
        x1: data[0][0],
        x2: data[0][1],
        y1: data[1][0],
        y2: data[1][1],
    }
}

fn simulate(target_area: &TargetArea, mut velocity_x: i32, mut velocity_y: i32) -> (i32, bool) {
    let mut x = 0;
    let mut y = 0;
    let success;
    let mut max_y = -i32::MAX;

    loop {
        // handle position
        x += velocity_x;
        y += velocity_y;

        // handle velocity X
        if velocity_x < 0 {
            velocity_x += 1;
        } else if velocity_x > 0 {
            velocity_x -= 1;
        }
        // handle velocity Y
        velocity_y -= 1;

        // track maximum y
        if max_y < y {
            max_y = y;
        }

        // check if target is hit
        if x >= target_area.x1 && x <= target_area.x2 && y >= target_area.y1 && y <= target_area.y2
        {
            success = true;
            break;
        }
        // check if already went pass target
        if x > target_area.x2 || y < target_area.y1 {
            success = false;
            break;
        }
    }

    (max_y, success)
}

fn part1() {
    let target_data = read_data();
    let mut max_y = -i32::MAX;
    let mut max_velocity_y = 0;
    let mut max_velocity_x = 0;

    for velocity_x in -200..200 {
        for velocity_y in -200..200 {
            let (y, success) = simulate(&target_data, velocity_x, velocity_y);
            if success && max_y < y {
                max_y = y;
                max_velocity_y = velocity_y;
                max_velocity_x = velocity_x;
            }
        }
    }

    println!(
        "Velocity X: {} Velocity Y: {} MAX Y: {}",
        max_velocity_x, max_velocity_y, max_y
    );
}

fn part2() {
    let target_data = read_data();
    let mut success_count = 0;
    
    for velocity_x in -200..200 {
        for velocity_y in -200..200 {
            let (_, success) = simulate(&target_data, velocity_x, velocity_y);
            if success {
                success_count += 1;
            }
        }
    }

    println!("Succeeded: {}", success_count);
}

fn main() {
    part1();
    part2();
}
