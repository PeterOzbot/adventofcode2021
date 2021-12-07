use std::fs;

struct BingoData {
    drawn_numbers: Vec<i32>,
    boards: Vec<BoardData>,
}

#[derive(Debug, Copy, Clone)]
struct CellData {
    drawn: bool,
    value: i32,
}

#[derive(Debug)]
struct BoardData {
    rows: Vec<Vec<CellData>>,
    columns: Vec<Vec<CellData>>,
}

fn read_bingo_data() -> BingoData {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut bingo_data = BingoData {
        drawn_numbers: vec![],
        boards: vec![],
    };

    for row in contents.split("\n") {
        // if no drawn numbers then first row is that
        if bingo_data.drawn_numbers.len() == 0 {
            bingo_data.drawn_numbers = row
                .split(",")
                .map(|number| number.parse::<i32>().unwrap())
                .collect();
        }
        // other rows are boards
        else {
            if !row.is_empty() {
                let board_row = row
                    .split(" ")
                    .filter(|value| !(*value).is_empty())
                    .map(|number| number.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let board_cells = board_row
                    .iter()
                    .map(|cell_value| CellData {
                        value: *cell_value,
                        drawn: false,
                    })
                    .collect();

                let board_index = bingo_data.boards.len() - 1;
                bingo_data.boards[board_index].rows.push(board_cells);
            } else {
                bingo_data.boards.push(BoardData {
                    columns: vec![],
                    rows: vec![],
                });
            }
        }
    }

    for board in bingo_data.boards.iter_mut() {
        for row in &board.rows {
            for (index, cell) in row.iter().enumerate() {
                match board.columns.get(index) {
                    Some(_) => (),
                    None => board.columns.push(vec![]),
                }

                board.columns[index].push(cell.clone());
            }
        }
    }

    bingo_data
}
fn check_if_bingo(board: &BoardData) -> bool {
    // check if any row or column is full
    let mut all_cells = true;
    for row in &board.rows {
        for cell in row {
            if !cell.drawn {
                all_cells = false;
            }
        }

        if all_cells {
            return true;
        } else {
            all_cells = true;
        }
    }

    all_cells = true;
    for row in &board.columns {
        for cell in row {
            if !cell.drawn {
                all_cells = false;
            }
        }

        if all_cells {
            return true;
        } else {
            all_cells = true;
        }
    }

    return false;
}
fn get_winning_board_score(bingo_data: &mut BingoData) -> i64 {
    for drawn_number in &bingo_data.drawn_numbers {
        // loop through boards
        for board in bingo_data.boards.iter_mut() {
            // check rows and columns for number
            for row in board.rows.iter_mut() {
                for cell in row {
                    if cell.value == *drawn_number {
                        cell.drawn = true;
                    }
                }
            }

            for column in board.columns.iter_mut() {
                for cell in column {
                    if cell.value == *drawn_number {
                        cell.drawn = true;
                    }
                }
            }

            if check_if_bingo(&board) {
                return calculate_score(&board, *drawn_number);
            }
        }
    }

    panic!("Board not found.")
}

fn calculate_score(board: &BoardData, drawn_number: i32) -> i64 {
    let mut sum: i64 = 0;

    for row in &board.rows {
        for cell in row {
            if !cell.drawn {
                sum += i64::from(cell.value);
            }
        }
    }

    sum * i64::from(drawn_number)
}

fn get_last_winning_board_score(bingo_data: &mut BingoData) -> i64 {
    let mut boards_won: Vec<*const BoardData> = vec![];
    let boards_count = bingo_data.boards.len();

    for drawn_number in &bingo_data.drawn_numbers {
        // loop through boards
        for board in bingo_data.boards.iter_mut() {
            // check rows and columns for number
            for row in board.rows.iter_mut() {
                for cell in row {
                    if cell.value == *drawn_number {
                        cell.drawn = true;
                    }
                }
            }

            for column in board.columns.iter_mut() {
                for cell in column {
                    if cell.value == *drawn_number {
                        cell.drawn = true;
                    }
                }
            }

            if check_if_bingo(&board) {
                let ptr = board as *const BoardData;
                if !boards_won.contains(&ptr) {
                    boards_won.push(ptr);
                } 
                if boards_won.len() == boards_count {
                    return calculate_score(&board, *drawn_number);
                }
            }
        }
    }

    panic!("Board not found.")
}

fn part1() {
    println!("Score: {}", get_winning_board_score(&mut read_bingo_data()))
}

fn part2() {
    println!(
        "Score: {}",
        get_last_winning_board_score(&mut read_bingo_data())
    );
}

fn main() {
    part1();
    part2()
}
