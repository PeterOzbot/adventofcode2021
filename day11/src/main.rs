use std::{fmt, fs};

struct Octopus {
    energy: u32,
    flashed: bool,
}

impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.energy)
    }
}

fn read_data() -> Vec<Vec<Octopus>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Octopus {
                    energy: c.to_digit(10).unwrap(),
                    flashed: false,
                })
                .collect()
        })
        .collect()
}

fn chain_flash(octopuses: &mut Vec<Vec<Octopus>>) -> i32 {
    let mut flash_count = 0;
    for x in 0..octopuses.len() {
        for y in 0..octopuses[0].len() {
            flash_count += check_flash(octopuses, x, y);
        }
    }
    flash_count
}

fn check_flash(octopuses: &mut Vec<Vec<Octopus>>, x: usize, y: usize) -> i32 {
    if !octopuses[x][y].flashed && octopuses[x][y].energy > 9 {
        octopuses[x][y].flashed = true;

        flash_around(octopuses, x, y) + 1
    } else {
        0
    }
}

fn flash_around(octopuses: &mut Vec<Vec<Octopus>>, x: usize, y: usize) -> i32 {
    let mut flash_count = 0;

    if x > 0 {
        //down
        octopuses[x - 1][y].energy += 1;
        flash_count += check_flash(octopuses, x - 1, y);
    }
    if x < 9 {
        //top
        octopuses[x + 1][y].energy += 1;
        flash_count += check_flash(octopuses, x + 1, y);
    }
    if y > 0 {
        // left
        octopuses[x][y - 1].energy += 1;
        flash_count += check_flash(octopuses, x, y - 1);
    }
    if y < 9 {
        // right
        octopuses[x][y + 1].energy += 1;
        flash_count += check_flash(octopuses, x, y + 1);
    }

    if x > 0 {
        if y > 0 {
            // top left
            octopuses[x - 1][y - 1].energy += 1;
            flash_count += check_flash(octopuses, x - 1, y - 1);
        }
        if y < 9 {
            // top right
            octopuses[x - 1][y + 1].energy += 1;
            flash_count += check_flash(octopuses, x - 1, y + 1);
        }
    }
    if x < 9 {
        if y > 0 {
            // bottom left
            octopuses[x + 1][y - 1].energy += 1;
            flash_count += check_flash(octopuses, x + 1, y - 1);
        }
        if y < 9 {
            // bottom right
            octopuses[x + 1][y + 1].energy += 1;
            flash_count += check_flash(octopuses, x + 1, y + 1);
        }
    }
    flash_count
}

fn increase_energy(octopuses: &mut Vec<Vec<Octopus>>) {
    for x in 0..octopuses.len() {
        for y in 0..octopuses[0].len() {
            if octopuses[x][y].energy > 9 {
                octopuses[x][y].energy = 0;
            }
            octopuses[x][y].energy += 1;
            octopuses[x][y].flashed = false;
        }
    }
}

fn process(octopuses: &mut Vec<Vec<Octopus>>) {
    let mut flash_count = 0;
    for step in 1..=100 {
        increase_energy(octopuses);
        flash_count += chain_flash(octopuses);
    }

    // for x in 0..octopuses.len() {
    //     println!("{:?}", octopuses[x])
    // }

    println!("Flashed: {}", flash_count);
}

fn find_full_flash(octopuses: &mut Vec<Vec<Octopus>>) {
    let full_flash_count = (octopuses.len() * octopuses.len()) as i32;
    let mut current_flash = 0i32;
    let mut step = 0;
    while current_flash != full_flash_count {
        increase_energy(octopuses);
        current_flash = chain_flash(octopuses);
        step += 1;
    }

    // for x in 0..octopuses.len() {
    //     println!("{:?}", octopuses[x])
    // }

    println!("All flashed at: {}", step);
}

fn part1() {
    process(&mut read_data());
}

fn part2() {
    find_full_flash(&mut read_data());
}

fn main() {
    part1();
    part2();
}
