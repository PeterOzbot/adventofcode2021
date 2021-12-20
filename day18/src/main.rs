use itertools::Itertools;
use std::{fs, str::Chars};

use models::SnailfishNumber;

mod models;

fn read_data() -> Vec<SnailfishNumber> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut snail_numbers = vec![];
    for line in contents.lines() {
        // trim first/last [,]
        let mut chars = line.chars();
        chars.next();
        chars.next_back();

        snail_numbers.push(parse_data(chars.as_str()));
    }

    snail_numbers
}

fn parse_data(line: &str) -> SnailfishNumber {
    let mut chars = line.chars();

    parse_chars(&mut chars)
}

fn parse_chars(chars: &mut Chars) -> SnailfishNumber {
    let mut left: SnailfishNumber = SnailfishNumber::Number(0);
    let mut right: SnailfishNumber = SnailfishNumber::Number(0);
    let mut comma = false;

    while let Some(char) = chars.next() {
        if char == ',' {
            comma = true;
        }
        if char.is_numeric() {
            if !comma {
                left = SnailfishNumber::Number(char.to_digit(10).unwrap() as u8)
            } else {
                right = SnailfishNumber::Number(char.to_digit(10).unwrap() as u8)
            }
        }
        if char == '[' {
            let sub_snail = parse_chars(chars);
            if !comma {
                left = sub_snail;
            } else {
                right = sub_snail;
            }
        }
        if char == ']' {
            return SnailfishNumber::Pair(Box::new(left), Box::new(right));
        }
    }

    return SnailfishNumber::Pair(Box::new(left), Box::new(right));
}

fn add_left(pair: SnailfishNumber, val: Option<u8>) -> SnailfishNumber {
    match val {
        None => pair,
        Some(v) => match pair {
            SnailfishNumber::Number(n) => SnailfishNumber::Number(n + v),
            SnailfishNumber::Pair(left, right) => {
                let new_left = add_left(*left, val);
                SnailfishNumber::Pair(Box::new(new_left), Box::new(*right))
            }
        },
    }
}

fn add_right(pair: SnailfishNumber, val: Option<u8>) -> SnailfishNumber {
    match val {
        None => pair,
        Some(v) => match pair {
            SnailfishNumber::Number(n) => SnailfishNumber::Number(n + v),
            SnailfishNumber::Pair(left, right) => {
                let new_right = add_right(*right, val);
                SnailfishNumber::Pair(Box::new(*left), Box::new(new_right))
            }
        },
    }
}

fn explode(pair: SnailfishNumber, depth: u8) -> (SnailfishNumber, Option<u8>, Option<u8>, bool) {
    match pair {
        SnailfishNumber::Number(_) => (pair, None, None, false),
        SnailfishNumber::Pair(a, b) => {
            if depth >= 4 {
                match (*a, *b) {
                    (SnailfishNumber::Number(a), SnailfishNumber::Number(b)) => {
                        return (SnailfishNumber::Number(0), Some(a), Some(b), true)
                    }
                    _ => unreachable!(),
                }
            }
            let (sub_pair_left, left_ret, left_add, exploded) = explode(*a, depth + 1);
            if exploded {
                let new_right = add_left(*b, left_add);
                (
                    SnailfishNumber::Pair(Box::new(sub_pair_left), Box::new(new_right)),
                    left_ret,
                    None,
                    true,
                )
            } else {
                let (sub_pair_right, right_add, right_ret, exploded) = explode(*b, depth + 1);
                let new_left = add_right(sub_pair_left, right_add);
                (
                    SnailfishNumber::Pair(Box::new(new_left), Box::new(sub_pair_right)),
                    None,
                    right_ret,
                    exploded,
                )
            }
        }
    }
}

fn split(pair: SnailfishNumber) -> (SnailfishNumber, bool) {
    match pair {
        SnailfishNumber::Number(n) => {
            if n >= 10 {
                let left = SnailfishNumber::Number(n / 2);
                let right = SnailfishNumber::Number(n / 2 + n % 2);
                (SnailfishNumber::Pair(Box::new(left), Box::new(right)), true)
            } else {
                (pair, false)
            }
        }

        SnailfishNumber::Pair(left, right) => {
            let (new_left, has_split) = split(*left);
            if has_split {
                (
                    SnailfishNumber::Pair(Box::new(new_left), Box::new(*right)),
                    true,
                )
            } else {
                let (new_right, has_split) = split(*right);
                (
                    SnailfishNumber::Pair(Box::new(new_left), Box::new(new_right)),
                    has_split,
                )
            }
        }
    }
}

fn magnitude(pair: &SnailfishNumber) -> u64 {
    match pair {
        SnailfishNumber::Number(n) => *n as _,
        SnailfishNumber::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

fn reduce(mut pair: SnailfishNumber) -> SnailfishNumber {
    loop {
        let (new_pair, _, _, exploded) = explode(pair, 0);
        pair = new_pair;
        if exploded {
            continue;
        }

        let (new_pair, has_split) = split(pair);
        pair = new_pair;
        if has_split {
            continue;
        }
        break;
    }
    pair
}

fn part1() {
    let reduced_numbers = read_data()
        .into_iter()
        .reduce(|first, second| reduce(SnailfishNumber::Pair(Box::new(first), Box::new(second))))
        .unwrap();

    let magnitude = magnitude(&reduced_numbers);
    println!("Magnitude: {:?}", magnitude);
}

fn part2() {
    let reduced_numbers = read_data()
        .into_iter()
        .permutations(2)
        .map(|permutation| {
            reduce(SnailfishNumber::Pair(
                Box::new(permutation[0].clone()),
                Box::new(permutation[1].clone()),
            ))
        })
        .max_by_key(magnitude)
        .unwrap();

    println!("Max Magnitude: {:?}", magnitude(&reduced_numbers));
}

fn main() {
    part1();
    part2();
}
