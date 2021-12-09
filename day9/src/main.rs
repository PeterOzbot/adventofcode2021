use std::fs;

fn read_data() -> Vec<Vec<u32>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_low_points(matrix: Vec<Vec<u32>>) -> Vec<u32> {
    let mut low_points: Vec<u32> = vec![];

    let height = matrix.len();
    let width = matrix[0].len();
    for x in 0..height {
        for y in 0..width {
            let number = matrix[x][y];

            // top
            if x > 0 && number >= matrix[x - 1][y] {
                continue;
            }

            // bottom
            if x < (height - 1) && number >= matrix[x + 1][y] {
                continue;
            }

            // left
            if y > 0 && number >= matrix[x][y - 1] {
                continue;
            }

            // right
            if y < (width - 1) && number >= matrix[x][y + 1] {
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

fn main() {
    part1();
}
