use std::fs;

fn read_data() -> Vec<usize> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    contents
        .split(",")
        .map(|number| number.parse::<usize>().unwrap())
        .collect()
}

fn part1() {
    let mut lanterns = read_data();
    let mut new_lanterns = 0;
    for _ in 1..=80 {
        for lantern in lanterns.iter_mut() {
            if *lantern == 0 {
                *lantern = 6;
                new_lanterns += 1;
            } else {
                *lantern -= 1;
            }
        }

        for _ in 0..new_lanterns {
            lanterns.push(8);
        }
        new_lanterns = 0;
    }

    println!("Total fish: {}", lanterns.len());
}

fn part2() {
    let lanterns = read_data();

    // fill ages
    let mut ages: Vec<i64> = vec![0; 9];
    for lantern_age in lanterns {
        ages[lantern_age] += 1;
    }

    // process fish
    for _ in 1..=256 {
        let mut new_ages: Vec<i64> = vec![0; 9];

        new_ages[8] = ages[0];
        new_ages[7] = ages[8];
        new_ages[6] = ages[7] + ages[0];
        new_ages[5] = ages[6];
        new_ages[4] = ages[5];
        new_ages[3] = ages[4];
        new_ages[2] = ages[3];
        new_ages[1] = ages[2];
        new_ages[0] = ages[1];

        ages = new_ages;
    }

    // count fish
    let mut total_fish = 0i64;
    for fish_count in ages {
        total_fish += fish_count;
    }

    println!("Total fish: {}", total_fish);
}

fn main() {
    part1();
    part2();
}
