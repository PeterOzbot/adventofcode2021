use std::{collections::HashMap, fs};

use models::Game;

use crate::models::{DeterministicDice, Player};

mod models;

fn read_data() -> (u16, u16) {
    let file_name = "input";
    let mut input = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    input = input.replace("Player 1 starting position: ", "");
    input = input.replace("Player 2 starting position: ", "");

    let mut input_lines = input.lines();
    let player1_input = input_lines.next().unwrap().parse::<u16>().unwrap();
    let player2_input = input_lines.next().unwrap().parse::<u16>().unwrap();

    (player1_input, player2_input)
}

fn update_player_score(player: &mut Player, roll_sum: u32) {
    player.position += roll_sum;
    player.position = (player.position - 1) % 10 + 1;
    player.score += player.position;
}

fn roll_dice(player1_position: u16, player2_position: u16) {
    let mut dice = DeterministicDice {
        roll_counter: 0,
        value: 0,
    };
    let mut player_1 = Player {
        position: player1_position as u32,
        score: 0,
    };
    let mut player_2 = Player {
        position: player2_position as u32,
        score: 0,
    };

    loop {
        let roll_sum = dice.roll();
        update_player_score(&mut player_1, roll_sum);
        if player_1.score >= 1000 {
            break;
        }

        let roll_sum = dice.roll();
        update_player_score(&mut player_2, roll_sum);
        if player_2.score >= 1000 {
            break;
        }
    }

    let loosing_score;
    if player_1.score > player_2.score {
        loosing_score = player_2.score;
    } else {
        loosing_score = player_1.score;
    }

    println!("Score: {}", dice.roll_counter * loosing_score)
}

fn part1() {
    let data = read_data();
    roll_dice(data.0, data.1);
}
fn part2() {
    let data = read_data();
    let game = Game::new(data.0, data.1);
    let result =game.get_wins(&mut HashMap::new()).into_iter().max().unwrap();
    println!("Max Winning games: {}", result)
}

fn main() {
    part1();
    part2();
}
