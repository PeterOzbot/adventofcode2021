use std::{collections::HashMap, fs};

fn read_data() -> Vec<Vec<String>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let cave_connections = contents
        .lines()
        .map(|line| line.split("-").map(|cave| cave.to_string()).collect())
        .collect();

    cave_connections
}

fn parse_to_tree(cave_connections: Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut cave_mappings: HashMap<String, Vec<String>> = HashMap::new();

    for connection in cave_connections {
        let connection1 = connection[0].clone();
        let connection2 = connection[1].clone();

        cave_mappings
            .entry(connection1)
            .and_modify(|caves| caves.push(connection2.clone()))
            .or_insert(vec![connection2]);
    }

    cave_mappings
}

fn build(cave_mappings: &mut HashMap<String, Vec<String>>) -> &HashMap<String, Vec<String>> {
    let keys = cave_mappings.keys().cloned().collect::<Vec<_>>();
    for cave in keys {
        let connections = cave_mappings.get(&cave).unwrap().clone();

        for connection in connections {
            let connection1 = connection.clone();
            let connection2 = cave.clone();

            cave_mappings
                .entry(connection1)
                .and_modify(|caves| {
                    if let None = caves.iter().find(|c| **c == connection2.clone()) {
                        caves.push(connection2.clone())
                    }
                })
                .or_insert(vec![connection2]);
        }
    }

    cave_mappings
}

fn build_caves(cave_mappings: &HashMap<String, Vec<String>>) {
    let mut visited_caves: Vec<Vec<String>> = vec![];
    deeper(
        cave_mappings,
        String::from("start"),
        &mut vec![String::from("start")],
        &mut visited_caves,
    );

    println!("{:?}", visited_caves.len());
}

fn deeper(
    cave_mappings: &HashMap<String, Vec<String>>,
    next: String,
    current_path: &mut Vec<String>,
    visited_caves: &mut Vec<Vec<String>>,
) {
    let next_caves = cave_mappings.get(&next).unwrap();

    for next_cave in next_caves {
        if next_cave == "end" {
            visited_caves.push(vec![String::from("start")]);
        }

        if next_cave == "start" || next_cave == "end" {
            continue;
        }

        if next_cave.chars().all(char::is_lowercase) {
            let searching_key = next_cave.clone();
            if let Some(_) = current_path.iter().find(|c| **c == searching_key) {
                continue;
            }
        }

        let mut new_path = current_path.clone();
        new_path.push(next_cave.clone());

        deeper(
            cave_mappings,
            next_cave.clone(),
            &mut new_path,
            visited_caves,
        );
    }
}

fn build_caves2(cave_mappings: &HashMap<String, Vec<String>>) {
    let mut visited_caves: Vec<Vec<String>> = vec![];
    deeper2(
        cave_mappings,
        String::from("start"),
        &mut vec![String::from("start")],
        &mut visited_caves,
    );

    println!("{:?}", visited_caves.len());
}

fn deeper2(
    cave_mappings: &HashMap<String, Vec<String>>,
    next: String,
    current_path: &mut Vec<String>,
    visited_caves: &mut Vec<Vec<String>>,
) {
    // check if small cave is already twice in path
    let locked = small_cave_repeats(current_path);

    let next_caves = cave_mappings.get(&next).unwrap();
    for next_cave in next_caves {
        if next_cave == "end" {
            visited_caves.push(vec![String::from("start")]);
        }

        if next_cave == "start" || next_cave == "end" {
            continue;
        }

        if next_cave.chars().all(char::is_lowercase) && locked {
            let searching_key = next_cave.clone();
            if let Some(_) = current_path.iter().find(|c| **c == searching_key) {
                continue;
            }
        }

        let mut new_path = current_path.clone();
        new_path.push(next_cave.clone());

        deeper2(
            cave_mappings,
            next_cave.clone(),
            &mut new_path,
            visited_caves,
        );
    }
}

fn small_cave_repeats(current_path: &Vec<String>) -> bool {
    let mut cave_hash: HashMap<String, u32> = HashMap::new();

    for cave in current_path.into_iter() {
        if cave.chars().all(char::is_lowercase) {
            cave_hash
                .entry(cave.clone())
                .and_modify(|cave_count| *cave_count += 1)
                .or_insert(1);
        }
    }
    
    // get max number of repeats
    let max = cave_hash
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(_, b)| b)
        .unwrap();

    max > 1
}

fn part1() {
    build_caves(build(&mut parse_to_tree(read_data())));
}

fn part2() {
    build_caves2(build(&mut parse_to_tree(read_data())));
}

fn main() {
    part1();
    part2();
}
