use std::{collections::HashMap, fs};

use models::Data;
mod models;

fn read_data() -> Data {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut process_pair_insertions = false;
    let mut polymer_template = vec![];
    let mut pair_insertions: HashMap<String, char> = HashMap::new();

    for line in contents.lines() {
        if line == "" {
            process_pair_insertions = true;
            continue;
        }
        if !process_pair_insertions {
            polymer_template = line.chars().collect();
        } else {
            let mut key: String = String::from("");
            let mut value: char = ' ';
            for data in line.split(" -> ") {
                if data.len() == 2 {
                    key = data.to_string();
                } else {
                    value = data.chars().nth(0).unwrap();
                }
            }
            pair_insertions.insert(key.clone(), value);
        }
    }

    Data {
        polymer_template: polymer_template,
        pair_insertions: pair_insertions,
    }
}

fn update_element_count(element: char, count: u128, element_count: &mut HashMap<char, u128>) {
    element_count
        .entry(element)
        .and_modify(|element_value| *element_value += count)
        .or_insert(1);
}

fn calculate_min_max(element_count: HashMap<char, u128>) {
    let mut min_count = u128::MAX;
    let mut max_count = 0;

    for (_, count) in element_count {
        if min_count > count {
            min_count = count;
        }
        if max_count < count {
            max_count = count;
        }
    }

    println!("Difference max-min: {:?}", max_count - min_count);
}

fn create_new_pairs(element: char, pair: &String) -> (String, String) {
    // split pair
    let mut chars = pair.chars();
    let first_element = chars.next().unwrap();
    let second_element = chars.next().unwrap();

    let mut first_pair = String::from(first_element);
    first_pair.push(element);

    let mut second_pair = String::from(element);
    second_pair.push(second_element);

    (first_pair, second_pair)
}

fn process(step: u32) {
    let mut element_count: HashMap<char, u128> = HashMap::new();
    let mut pair_count: HashMap<String, u128> = HashMap::new();

    let data = read_data();

    // initialize with initial pairs
    for pair in data.polymer_template.windows(2) {
        let pair_str = pair.into_iter().collect();
        *pair_count.entry(pair_str).or_default() += 1;
    }

    // count initial elements
    for element in data.polymer_template {
        update_element_count(element, 1, &mut element_count);
    }

    for _ in 1..=step {
        let values = pair_count.clone();

        for (pair, count) in values {
            // get element from pair and update count
            let element = data.pair_insertions.get(&pair).unwrap();
            update_element_count(*element, count, &mut element_count);

            // create new pairs
            let (new_first_pair, new_second_pair) = create_new_pairs(*element, &pair);

            // reduce the existing pair count -> we create new pairs from it
            *pair_count.entry(pair).or_default() -= count;

            // increase new pair counts
            *pair_count.entry(new_first_pair).or_default() += count;
            *pair_count.entry(new_second_pair).or_default() += count;
        }
    }

    calculate_min_max(element_count);
}

fn part1() {
    process(10);
}
fn part2() {
    process(40);
}

fn main() {
    part1();
    part2();
}
