use std::{collections::LinkedList, fs, vec};

fn read_data() -> Vec<Vec<char>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_illegal_brackets(data: Vec<Vec<char>>) -> Vec<char> {
    let mut illegal_brackets: Vec<char> = vec![];
    for line in data {
        let mut open_bracket_stack: LinkedList<char> = LinkedList::new();
        for bracket in line {
            let (matched, incomplete) = match bracket {
                '{' => {
                    open_bracket_stack.push_back(bracket);
                    (true, true)
                }
                '(' => {
                    open_bracket_stack.push_back(bracket);
                    (true, true)
                }
                '[' => {
                    open_bracket_stack.push_back(bracket);
                    (true, true)
                }
                '<' => {
                    open_bracket_stack.push_back(bracket);
                    (true, true)
                }

                '}' => check_closing_bracket(bracket, &mut open_bracket_stack),
                ')' => check_closing_bracket(bracket, &mut open_bracket_stack),
                ']' => check_closing_bracket(bracket, &mut open_bracket_stack),
                '>' => check_closing_bracket(bracket, &mut open_bracket_stack),
                _ => unreachable!(),
            };

            if !incomplete {
                panic!("WTF");
            } else if !matched {
                illegal_brackets.push(bracket);
                break;
            }
        }
    }

    illegal_brackets
}

fn check_closing_bracket(
    closing_bracket: char,
    open_bracket_stack: &mut LinkedList<char>,
) -> (bool, bool) {
    let open_bracket_value = open_bracket_stack.pop_back();
    if let Some(open_bracket) = open_bracket_value {
        let matched = match closing_bracket {
            '}' => open_bracket == '{',
            ')' => open_bracket == '(',
            ']' => open_bracket == '[',
            '>' => open_bracket == '<',
            _ => unreachable!(),
        };
        (matched, true)
    } else {
        (false, false)
    }
}

fn calculate_score_illegal(illegal_brackets: Vec<char>) {
    let score: i64 = illegal_brackets
        .into_iter()
        .map(|character| match character {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum();
    println!("Total score: {}", score);
}

fn part1() {
    calculate_score_illegal(find_illegal_brackets(read_data()));
}

struct IncompleteLine {
    open_bracket_stack: LinkedList<char>,
}

fn find_incomplete(data: Vec<Vec<char>>) -> Vec<IncompleteLine> {
    let mut incomplete_lines: Vec<IncompleteLine> = vec![];

    for line in data {
        let mut open_bracket_stack: LinkedList<char> = LinkedList::new();
        for bracket in &line {
            let (matched, incomplete) = match bracket {
                '{' => {
                    open_bracket_stack.push_back(*bracket);
                    (true, true)
                }
                '(' => {
                    open_bracket_stack.push_back(*bracket);
                    (true, true)
                }
                '[' => {
                    open_bracket_stack.push_back(*bracket);
                    (true, true)
                }
                '<' => {
                    open_bracket_stack.push_back(*bracket);
                    (true, true)
                }

                '}' => check_closing_bracket(*bracket, &mut open_bracket_stack),
                ')' => check_closing_bracket(*bracket, &mut open_bracket_stack),
                ']' => check_closing_bracket(*bracket, &mut open_bracket_stack),
                '>' => check_closing_bracket(*bracket, &mut open_bracket_stack),
                _ => unreachable!(),
            };

            if !incomplete {
                panic!("WTF");
            } else if !matched {
                open_bracket_stack.clear();
                break;
            }
        }

        if !open_bracket_stack.is_empty() {
            incomplete_lines.push(IncompleteLine {
                open_bracket_stack: open_bracket_stack,
            });
        }
    }

    incomplete_lines
}

fn complete_lines(incomplete_lines: &mut Vec<IncompleteLine>) -> Vec<Vec<char>> {
    let mut complete_braces: Vec<Vec<char>> = vec![];
    for line in incomplete_lines {
        let mut line_complete_braces: Vec<char> = vec![];
        while !line.open_bracket_stack.is_empty() {
            line_complete_braces.push(line.open_bracket_stack.pop_back().unwrap());
        }
        complete_braces.push(line_complete_braces);
    }

    complete_braces
}

fn calculate_score_missing(missing_brackets: Vec<char>) -> i64 {
    let mut sum = 0;
    for missing_bracket in missing_brackets {
        sum = sum * 5
            + match missing_bracket {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };
    }
    sum
}

fn find_medium(complete_braces: Vec<Vec<char>>) {
    let mut score: Vec<i64> = vec![];
    for braces in complete_braces {
        score.push(calculate_score_missing(braces));
    }

    score.sort();
    let mid = score.len() / 2;
    println!("{:?}", score[mid]);
}

fn part2() {
    find_medium(complete_lines(&mut find_incomplete(read_data())));
}

fn main() {
    part1();
    part2();
}
