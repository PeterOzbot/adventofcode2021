use std::fs;

#[derive(Debug)]
struct Cell {
    visited: bool,
    value: u32,
}

fn read_data() -> Vec<Vec<Cell>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell {
                    value: c.to_digit(10).unwrap(),
                    visited: false,
                })
                .collect()
        })
        .collect()
}

fn get_low_points(matrix: Vec<Vec<Cell>>) -> Vec<u32> {
    let mut low_points: Vec<u32> = vec![];

    let height = matrix.len();
    let width = matrix[0].len();
    for x in 0..height {
        for y in 0..width {
            let number = matrix[x][y].value;

            // top
            if x > 0 && number >= matrix[x - 1][y].value {
                continue;
            }

            // bottom
            if x < (height - 1) && number >= matrix[x + 1][y].value {
                continue;
            }

            // left
            if y > 0 && number >= matrix[x][y - 1].value {
                continue;
            }

            // right
            if y < (width - 1) && number >= matrix[x][y + 1].value {
                continue;
            }

            low_points.push(number);
        }
    }

    low_points
}

fn calculate_risk(low_points: Vec<u32>) {
    println!(
        "Risk: {}",
        low_points.iter().map(|point| point + 1).sum::<u32>()
    );
}

fn part1() {
    calculate_risk(get_low_points(read_data()));
}

fn get_basins(matrix: &mut Vec<Vec<Cell>>) -> Vec<u32> {
    let mut basins_size: Vec<u32> = vec![];

    let height = matrix.len();
    let width = matrix[0].len();
    for x in 0..height {
        for y in 0..width {
            let number = &matrix[x][y].value;

            // top
            if x > 0 && number >= &matrix[x - 1][y].value {
                continue;
            }

            // bottom
            if x < (height - 1) && number >= &matrix[x + 1][y].value {
                continue;
            }

            // left
            if y > 0 && number >= &matrix[x][y - 1].value {
                continue;
            }

            // right
            if y < (width - 1) && number >= &matrix[x][y + 1].value {
                continue;
            }

            let size = spread_basin(x, y, *number, matrix);
            basins_size.push(size);
        }
    }

    basins_size
}

fn spread_basin(start_x: usize, start_y: usize, number: u32, matrix: &mut Vec<Vec<Cell>>) -> u32 {
    let height = matrix.len();
    let width = matrix[0].len();
    let mut size: u32 = 1;

    matrix[start_x][start_y].visited = true;

    if start_x > 0 {
        let bottom_x = start_x - 1;
        let bottom_neighbor = &matrix[bottom_x][start_y];
        if !bottom_neighbor.visited && bottom_neighbor.value != 9 && number < bottom_neighbor.value
        {
            size += spread_basin(bottom_x, start_y, bottom_neighbor.value, matrix);
        }
    }

    if start_x < (height - 1) {
        let top_x = start_x + 1;
        let top_neighbor = &matrix[top_x][start_y];
        if !top_neighbor.visited && top_neighbor.value != 9 && number < top_neighbor.value {
            size += spread_basin(top_x, start_y, top_neighbor.value, matrix);
        }
    }

    if start_y > 0 {
        let left_y = start_y - 1;
        let left_neighbor = &matrix[start_x][left_y];
        if !left_neighbor.visited && left_neighbor.value != 9 && number < left_neighbor.value {
            size += spread_basin(start_x, left_y, left_neighbor.value, matrix);
        }
    }

    if start_y < (width - 1) {
        let right_x = start_y + 1;
        let right_neighbor = &matrix[start_x][right_x];
        if !right_neighbor.visited && right_neighbor.value != 9 && number < right_neighbor.value {
            size += spread_basin(start_x, right_x, right_neighbor.value, matrix);
        }
    }

    size
}

fn get_size_score(basins_size: &mut Vec<u32>) {
    basins_size.sort();
    let mut score: u32 = 1;
    for basin in basins_size.iter().rev().take(3) {
        score *= *basin;
    }
    println!("Basins score: {}", score);
}

fn part2() {
    get_size_score(&mut get_basins(&mut read_data()));
}

fn main() {
    part1();
    part2();
}
