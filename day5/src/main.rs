use std::fs;

fn read_data() -> Vec<Vec<Vec<usize>>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let data: Vec<Vec<Vec<usize>>> = contents
        .lines()
        .map(|row| {
            row.split("->")
                .map(|coordinates| {
                    coordinates
                        .trim()
                        .split(",")
                        .map(|number| number.parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    data
}

fn exclude_irrelevant(data: Vec<Vec<Vec<usize>>>) -> Vec<Vec<Vec<usize>>> {
    let mut filtered_data = vec![];
    for row in data {
        let x1 = row[0][0];
        let y1 = row[0][1];
        let x2 = row[1][0];
        let y2 = row[1][1];

        if x1 == x2 || y1 == y2 {
            filtered_data.push(vec![vec![x1, y1], vec![x2, y2]])
        }
    }

    filtered_data
}

fn fill_matrix(data: Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
    let mut area_matrix = vec![vec![0; 1000]; 1000];
    for row in data {
        let no_data = match row[0].get(0) {
            Some(_) => false,
            None => true,
        };
        if no_data {
            continue;
        }

        let mut x1 = row[0][0];
        let mut y1 = row[0][1];
        let x2 = row[1][0];
        let y2 = row[1][1];

        while x1 != x2 || y1 != y2 {
            area_matrix[y1][x1] += 1;
            if x1 < x2 {
                x1 += 1;
            }else if x1 > x2{
                x1 -=1;
            }

            if y1 < y2 {
                y1 += 1;
            }else if y1 > y2{
                y1 -=1;
            }
        }
        area_matrix[y1][x1] += 1;
    }
    area_matrix
}

fn count_dangerous_points(area_matrix: Vec<Vec<usize>>) -> i32 {
    let mut dangerous_count = 0;
    for x in 0..area_matrix.len() {
        for y in 0..area_matrix[x].len() {
            if area_matrix[x][y] >= 2 {
                dangerous_count += 1;
            }
        }
    }
    dangerous_count
}

fn part1(){
    println!("{}",count_dangerous_points( fill_matrix(exclude_irrelevant(read_data()))));
}
fn part2(){
    println!("{}",count_dangerous_points( fill_matrix(read_data())));
}

fn main() {
    part1();
    part2();
 
 
}
