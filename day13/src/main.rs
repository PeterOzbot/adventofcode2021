use std::{collections::HashSet, fs};

use models::{Coordinate, Data, FoldInstruction, FoldType};
mod models;

fn read_data() -> Data {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut process_fold_instructions = false;
    let mut coordinates = vec![];
    let mut fold_instructions = vec![];

    for line in contents.lines() {
        if line == "" {
            process_fold_instructions = true;
            continue;
        }

        if !process_fold_instructions {
            let coordinates_line = line.split(",").collect::<Vec<&str>>();
            coordinates.push(Coordinate {
                x: coordinates_line[0].parse::<u32>().unwrap(),
                y: coordinates_line[1].parse::<u32>().unwrap(),
            });
        } else {
            let fold_data_line = line.replace("fold along ", "");
            let fold_data = fold_data_line.split("=").collect::<Vec<&str>>();

            let fold_type = match fold_data[0] {
                "x" => FoldType::X,
                "y" => FoldType::Y,
                _ => unreachable!(),
            };
            fold_instructions.push(FoldInstruction {
                fold_type: fold_type,
                value: fold_data[1].parse::<u32>().unwrap(),
            });
        }
    }

    Data {
        dots_coordinates: coordinates,
        fold_instructions: fold_instructions,
    }
}

fn fold_once(
    current_coordinates: Vec<Coordinate>,
    fold_instruction: &FoldInstruction,
) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = vec![];

    for coordinate in current_coordinates {
        match fold_instruction.fold_type {
            FoldType::X => {
                if coordinate.x < fold_instruction.value {
                    coordinates.push(Coordinate {
                        x: coordinate.x,
                        y: coordinate.y,
                    })
                } else if coordinate.x > fold_instruction.value {
                    let new_x = fold_instruction.value - (coordinate.x - fold_instruction.value);
                    coordinates.push(Coordinate {
                        x: new_x,
                        y: coordinate.y,
                    })
                } else {
                    panic!("WTF");
                }
            }
            FoldType::Y => {
                if coordinate.y < fold_instruction.value {
                    coordinates.push(Coordinate {
                        x: coordinate.x,
                        y: coordinate.y,
                    })
                } else if coordinate.y > fold_instruction.value {
                    let new_y = fold_instruction.value - (coordinate.y - fold_instruction.value);
                    coordinates.push(Coordinate {
                        x: coordinate.x,
                        y: new_y,
                    })
                } else {
                    panic!("WTF");
                }
            }
        }
    }

    coordinates
}

fn process1(data: Data) -> Vec<Coordinate> {
    let fold_instr = &data.fold_instructions[0];
    fold_once(data.dots_coordinates, fold_instr)
}

fn count(coordinates: Vec<Coordinate>) {
    let mut coordinates_mapped: HashSet<String> = HashSet::new();
    for coordinate in coordinates {
        let coordinate_display = format!("{:?}", coordinate);
        if !coordinates_mapped.contains(&coordinate_display) {
            coordinates_mapped.insert(coordinate_display);
        }
    }
    println!("Dots: {}", coordinates_mapped.len());
}

fn process2(data: Data) -> Vec<Coordinate> {
    let mut coordinates = data.dots_coordinates;
    for instruction in data.fold_instructions {
        coordinates = fold_once(coordinates, &instruction);
    }
    coordinates
}

fn print(coordinates: Vec<Coordinate>) {
    // the size may be searched, but for simplicity....its hardcoded
    let mut coordinates_grid: Vec<Vec<char>> = vec![vec!['.'; 39]; 6];
    for coordinate in coordinates {
        coordinates_grid[coordinate.y as usize][coordinate.x as usize] = '#';
    }

    for y in 0..coordinates_grid.len() {
        for x in 0..coordinates_grid[y].len(){
            print!("{:?}", coordinates_grid[y][x]);
        }
        println!("");
    }
}

fn part1() {
    count(process1(read_data()));
}

fn part2() {
    print(process2(read_data()));
}

fn main() {
    part1();
    part2();
}
