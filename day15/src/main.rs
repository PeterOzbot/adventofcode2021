use std::fs;

use priority_queue::PriorityQueue;

use crate::models::VisitedNode;

mod models;

fn read_data(large: bool) -> Vec<Vec<u64>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut data = vec![];

    for line in contents.lines() {
        let source_row: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        if large {
            let mut row = vec![];
            row.append(&mut source_row.clone());

            for projection in 1..=4 {
                for digit in &source_row {
                    let mut new_digit = digit + projection;
                    if new_digit > 9 {
                        new_digit = new_digit - 9;
                    }
                    row.push(new_digit);
                }
            }

            data.push(row);
        } else {
            data.push(source_row);
        }
    }

    if large {
        let size = data.len();
        for projection in 1..=4 {
            for row in 0..size {
                let mut new_row = vec![];

                for digit in &data[row] {
                    let mut new_digit = digit + projection;
                    if new_digit > 9 {
                        new_digit = new_digit - 9;
                    }
                    new_row.push(new_digit);
                }

                data.push(new_row);
            }
        }
    }

    data
}

fn initialize_grids(size: usize) -> (Vec<Vec<u64>>, Vec<Vec<bool>>) {
    let mut dist_map: Vec<Vec<u64>> = vec![];
    let mut visited: Vec<Vec<bool>> = vec![];

    for _ in 0..size {
        let mut dist_row = vec![];
        let mut visited_row = vec![];
        for _ in 0..size {
            dist_row.push(u64::MAX);
            visited_row.push(false);
        }
        dist_map.push(dist_row);
        visited.push(visited_row);
    }

    (dist_map, visited)
}

fn find_path(map: Vec<Vec<u64>>) {
    let mut nodes = PriorityQueue::new();

    let map_size = map.len();
    let (mut dist_map, mut visited) = initialize_grids(map_size);

    let mut x = 0;
    let mut y = 0;

    // set start map distance to zero and add to queue
    dist_map[0][0] = 0;
    nodes.push(VisitedNode { x: 0, y: 0 }, 0);

    // loop until destination is reached
    loop {
        // # move to x+1,y
        if x < map_size - 1 {
            let target_x = x + 1;
            let distance = map[target_x][y] + dist_map[x][y];

            if dist_map[target_x][y] > distance && !visited[target_x][y] {
                dist_map[target_x][y] = distance;
                nodes.push(VisitedNode { x: target_x, y: y }, -(distance as i32));
            }
        }

        //move to x-1,y
        if x > 0 {
            let target_x = x - 1;
            let distance = map[target_x][y] + dist_map[x][y];

            if dist_map[target_x][y] > distance && !visited[target_x][y] {
                dist_map[target_x][y] = distance;
                nodes.push(VisitedNode { x: target_x, y: y }, -(distance as i32));
            }
        }

        //move to x,y+1
        if y < map_size - 1 {
            let target_y = y + 1;
            let distance = map[x][target_y] + dist_map[x][y];

            if dist_map[x][target_y] > distance && !visited[x][target_y] {
                dist_map[x][target_y] = distance;
                nodes.push(VisitedNode { x: x, y: target_y }, -(distance as i32));
            }
        }

        //move to x,y-1
        if y > 0 {
            let target_y = y - 1;
            let distance = map[x][target_y] + dist_map[x][y];

            if dist_map[x][target_y] > distance && !visited[x][target_y] {
                dist_map[x][target_y] = distance;
                nodes.push(VisitedNode { x: x, y: target_y }, -(distance as i32));
            }
        }

        // mark node as visited
        visited[x][y] = true;

        // find nearest node
        let min_x;
        let min_y;

        loop {
            let (min_node, _) = nodes.pop().unwrap();
            if !visited[min_node.x][min_node.y] {
                min_x = min_node.x;
                min_y = min_node.y;
                break;
            }
        }

        // update coordinates with nearest node so we continue search from there
        x = min_x;
        y = min_y;

        // stop if reached destination
        if x == map_size - 1 && y == map_size - 1 {
            break;
        }
    }

    println!("{:?}", dist_map[map_size - 1][map_size - 1]);
}

fn part1() {
    let map = read_data(false);
    find_path(map);
}

fn part2() {
    let map = read_data(true);
    find_path(map);
}

fn main() {
    part1();
    part2();
}
