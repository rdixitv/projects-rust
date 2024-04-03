use std::io;
use rand::prelude::*;

pub enum RPSError {
    ParseError(String),
    InputError,
}

#[derive(PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
    Other(String)
}

enum RPSResult {
    Win,
    Lose,
    Draw
}


fn flush() {
    io::Write::flush(&mut io::stdout()).expect("failed to flush stdout");
}

fn game(player_choice: &Choice, computer_choice: &Choice) -> RPSResult {
    match (player_choice, computer_choice) {
        (c1, c2) if (c1 == c2) => RPSResult::Draw,
        (Choice::Rock, Choice::Scissors) | (Choice::Scissors, Choice::Paper) | (Choice::Paper, Choice::Rock) => RPSResult::Win,
        _ => RPSResult::Lose
    }
}

pub fn run() -> Result<(), RPSError> {
    let mut rng = thread_rng();

    let best_of: u8;
    let mut player_choice_char: char;
    let mut player_choice: Choice;
    let mut computer_choice: Choice = Choice::Other("".to_string());
    let mut n: u8;
    let mut result: RPSResult;
    let mut input = String::new();
    let args: Vec<String> = std::env::args().collect();

    let mut res: Result<(), RPSError> = Ok(());

    let mut score: (u8, u8) = (0, 0);

    if args.len() == 2 {
        best_of = args[1].parse().unwrap_or_else(|_| {
            res = Err(RPSError::ParseError(args[1].clone()));
            return 3;
        })
    } else {
        print!("Best of: ");
        flush();
        io::stdin().read_line(&mut input).unwrap_or_else(|_| {
            res = Err(RPSError::InputError);
            return 10;
        });

        best_of = input.parse().unwrap_or_else(|_| {
            res = Err(RPSError::ParseError(input.clone()));
            return 3;
        })
    }

    match res {
        Err(_) => return res,
        _ => (),
    }

    for _ in 1..=best_of {
        print!("Rock, paper, or scissors [r/p/s]? ");
        flush();
        io::stdin().read_line(&mut input).unwrap_or_else(|_| {
            res = Err(RPSError::InputError);
            return 1;
        });
        player_choice_char = input
            .trim()
            .chars()
            .nth(0)
            .expect("Input should be 1 character");

        player_choice = match player_choice_char {
            'r' => Choice::Rock,
            'p' => Choice::Paper,
            's' => Choice::Scissors,
            other => Choice::Other(other.to_string()),
        };
        
        n = rng.gen_range(0..3);

        println!("Computer played: {}", match n {
            0 => {
                computer_choice = Choice::Rock;
                "Rock"
            }
            1 => {
                computer_choice = Choice::Paper;
                "Paper"
            }
            2 => {
                computer_choice = Choice::Scissors;
                "Scissors"
            }
            _ => ""
        });

        result = game(&player_choice, &computer_choice);

        println!("Round {}!\nYou: {}\nComputer: {}\n", match result {
            RPSResult::Win => {
                score.0 += 1;
                "won"
            },
            RPSResult::Lose => {
                score.1 += 1;
                "lost"
            },
            RPSResult::Draw => "drawn",
        }, score.0, score.1);

        if score.0 > best_of / 2 || score.1 > best_of / 2 {
            break;
        }
    }

    println!("Game {} {} to {}!", (if score.0 > score.1 {"won"} else if score.0 < score.1 {"lost"} else {"drawn"}), score.0, score.1);

    Ok(())
}
