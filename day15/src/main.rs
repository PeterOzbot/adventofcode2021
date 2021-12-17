use models::Node;
use std::fs;
use std::hash::{Hash, Hasher};

use priority_queue::PriorityQueue;

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
                let mut new_row=vec![];

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

    for x in 0..data.len() {
        for y in 0..data[0].len() {
            print!("{}", data[x][y]);
        }
        println!("");
    }

    data
}
struct VisitedNode {
    pub x: usize,
    pub y: usize,
}
impl PartialEq for VisitedNode {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.y && self.y == other.y
    }
}
impl Eq for VisitedNode {}

impl Hash for VisitedNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn part1() {
    let map = read_data(false);
    let mut dist_map: Vec<Vec<u64>> = vec![];
    let mut visited: Vec<Vec<bool>> = vec![];
    let mut finished = false;
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let max_val = 50;
    for _ in 0..max_val {
        let mut dist_row = vec![];
        let mut visited_row = vec![];
        for _ in 0..max_val {
            dist_row.push(u64::MAX);
            visited_row.push(false);
        }
        dist_map.push(dist_row);
        visited.push(visited_row);
    }
    dist_map[0][0] = 0;

    while !finished {
        // # move to x+1,y
        if x < max_val - 1 {
            if dist_map[x + 1][y] > map[x + 1][y] + dist_map[x][y] && !visited[x + 1][y] {
                dist_map[x + 1][y] = map[x + 1][y] + dist_map[x][y];
            }
        }

        //move to x-1,y
        if x > 0 {
            if dist_map[x - 1][y] > map[x - 1][y] + dist_map[x][y] && !visited[x - 1][y] {
                dist_map[x - 1][y] = map[x - 1][y] + dist_map[x][y];
            }
        }

        //move to x,y+1
        if y < max_val - 1 {
            if dist_map[x][y + 1] > map[x][y + 1] + dist_map[x][y] && !visited[x][y + 1] {
                dist_map[x][y + 1] = map[x][y + 1] + dist_map[x][y];
            }
        }

        //move to x,y-1
        if y > 0 {
            if dist_map[x][y - 1] > map[x][y - 1] + dist_map[x][y] && !visited[x][y - 1] {
                dist_map[x][y - 1] = map[x][y - 1] + dist_map[x][y];
            }
        }

        visited[x][y] = true;

        let mut min_dist = u64::MAX;
        let mut min_x = 0;
        let mut min_y = 0;

        for x1 in 0..max_val {
            for y1 in 0..max_val {
                if !visited[x1][y1] {
                    if min_dist > dist_map[x1][y1] {
                        min_dist = dist_map[x1][y1];
                        min_x = x1;
                        min_y = y1;
                    }
                }
            }
        }

        x = min_x;
        y = min_y;
        if x == max_val - 1 && y == max_val - 1 {
            finished = true;
        }
        count = count + 1;
    }

    println!("{:?}", dist_map[max_val - 1][max_val - 1]);
}

fn part2() {
    let mut nodes = PriorityQueue::new();
    let map = read_data(true);
    let mut dist_map: Vec<Vec<u64>> = vec![];
    let mut visited: Vec<Vec<bool>> = vec![];
    let mut finished = false;
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let max_val = map.len();
    for _ in 0..max_val {
        let mut dist_row = vec![];
        let mut visited_row = vec![];
        for _ in 0..max_val {
            dist_row.push(u64::MAX);
            visited_row.push(false);
        }
        dist_map.push(dist_row);
        visited.push(visited_row);
    }
    dist_map[0][0] = 0;
    nodes.push(VisitedNode { x: 0, y: 0 }, 0);

    while !finished {
        // # move to x+1,y
        if x < max_val - 1 {
            let dist = map[x + 1][y] + dist_map[x][y];
            if dist_map[x + 1][y] > dist && !visited[x + 1][y] {
                dist_map[x + 1][y] = dist;
                nodes.push(VisitedNode { x: x + 1, y: y }, -(dist as i32));
            }
        }

        //move to x-1,y
        if x > 0 {
            let dist = map[x - 1][y] + dist_map[x][y];
            if dist_map[x - 1][y] > dist && !visited[x - 1][y] {
                dist_map[x - 1][y] = dist;
                nodes.push(VisitedNode { x: x - 1, y: y }, -(dist as i32));
            }
        }

        //move to x,y+1
        if y < max_val - 1 {
            let dist = map[x][y + 1] + dist_map[x][y];
            if dist_map[x][y + 1] > dist && !visited[x][y + 1] {
                dist_map[x][y + 1] = dist;
                nodes.push(VisitedNode { x: x, y: y + 1 }, -(dist as i32));
            }
        }

        //move to x,y-1
        if y > 0 {
            let dist = map[x][y - 1] + dist_map[x][y];
            if dist_map[x][y - 1] > dist && !visited[x][y - 1] {
                dist_map[x][y - 1] = dist;
                nodes.push(VisitedNode { x: x, y: y - 1 }, -(dist as i32));
            }
        }

        visited[x][y] = true;

        let mut min_dist = u64::MAX;
        let mut min_x = 0;
        let mut min_y = 0;

        let mut found_min = false;
        while !found_min {
            let (min_node, _) = nodes.pop().unwrap();
            if !visited[min_node.x][min_node.y] {
                min_x = min_node.x;
                min_y = min_node.y;
                found_min = true;
            }
        }

        x = min_x;
        y = min_y;

        if x == max_val - 1 && y == max_val - 1 {
            finished = true
        }
        count = count + 1
    }

    println!("{:?}", dist_map[max_val - 1][max_val - 1]);
}

fn main() {
    //part1();
    part2();
}
