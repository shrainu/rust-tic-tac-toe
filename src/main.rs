use std::io;
use std::io::{stdin, stdout, Write};
use crate::Action::{No, Place, Quit, Unknown, Yes};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use ansi_term;
use crate::State::{AiTurn, GameOver, PlayerTurn};
use crate::WinCondition::Tie;

fn get_line(msg : &str) -> io::Result<String> {

    if msg != "" {
        print!("{}", msg);
        stdout().flush().unwrap();
    }

    let mut line = String::new();

    if let Err(e) = stdin().read_line(&mut line) {
        println!("ERROR : {}", e);
        return Err(e);
    }

    line.remove(line.len() - 1);

    return Ok(line);
}

fn parse_number(line : &String) -> Result<i32, String> {

    return match line.parse::<i32>() {
        Ok(i) => {
            Ok(i)
        }
        Err(_) => {
            let err = format!("ERROR: Could not parse i32 from `{}`", line);
            Err(err)
        }
    }
}

const WINDOWS : bool = false;

const TOKEN_X : i32 = 1;
const TOKEN_O : i32 = 2;

static mut TABLE: [i32;9] = [0; 9];

fn set_table(index : usize, value : i32) {
    unsafe {
        TABLE[index] = value;
    }
}

fn get_table(index : usize) -> i32 {
    unsafe {
        return TABLE[index];
    }
}

static mut RUNNING : bool = false;

fn get_running() -> bool {
    unsafe {
        return RUNNING;
    }
}

fn set_running(running : bool) {
    unsafe {
        RUNNING = running;
    }
}

enum Action {
    Unknown,
    Quit,
    Yes,
    No,
    Place(i32),
}

fn get_action() -> Action {

    let input = get_line(">> ");

    let line = match input {
        Ok(mut l) => l,
        _ => String::new()
    };

    if line.to_lowercase() == "quit" {
        return Quit;
    } else if line.to_lowercase() == "yes" {
        return Yes;
    } else if line.to_lowercase() == "no" {
        return No;
    } else if let Ok(i) = parse_number(&line) {
        return Place(i);
    } else {
        return Unknown;
    }
}

fn pick_tokens(rnd: &mut ThreadRng) -> (i32, i32) {

    let player_token;
    let ai_token;

    if rnd.gen_range(0..100) < 50 {
        player_token = TOKEN_X;
        ai_token = TOKEN_O;
    } else {
        player_token = TOKEN_O;
        ai_token = TOKEN_X;
    }

    return (player_token, ai_token);
}

fn pick_starter(rnd : &mut ThreadRng) -> i32 {

    if rnd.gen_range(0..100) < 50 {
        return TOKEN_X;
    } else {
        return TOKEN_O;
    }
}

fn draw_table(player : i32, ai : i32) {

    println!("+---+---+---+");

    let player_token = if player == TOKEN_X { "X" } else { "O" };
    let ai_token = if ai == TOKEN_X { "X" } else { "O" };

    for y in 0..3 {

        print!("| ");

        for x in 0..3 {
            let token = get_table(y * 3 + x);
            if token == player {
                print!(
                    "{}",
                    ansi_term::Colour::Green
                        .bold()
                        .paint(player_token)
                );
            } else if token == ai {
                print!(
                    "{}",
                    ansi_term::Colour::Red
                        .bold()
                        .paint(ai_token)
                );
            } else {
                print!(
                    "{}",
                    ansi_term::Colour::White
                        .dimmed()
                        .paint(
                            String::from((y * 3 + x).to_string())
                        )
                );
            }
            print!(" ");
            if x < 2 {
                print!("| ");
            }
        }

        print!("|\n");
        println!("+---+---+---+");
    }
}

enum State {
    PlayerTurn,
    AiTurn,
    GameOver(WinCondition),
}

enum WinCondition {
    Tie,
    Player,
    Ai,
}

fn main() {

/*    if WINDOWS {
        let enabled = ansi_term::enable_ansi_support();
    }*/

    let mut rnd = rand::thread_rng();

    let (mut ai, mut player) = pick_tokens(&mut rnd);

    let mut available_tiles = 9;

    let mut state = if pick_starter(&mut rnd) == player {
        State::PlayerTurn
    } else {
        State::AiTurn
    };

    set_running(true);

    while get_running() {

        if get_running() {
            draw_table(player, ai);
        }

        let mut action = Action::Unknown;

        match &mut state {
            PlayerTurn => {
                println!("Enter the position.");
                action = get_action();
                if let Place(i) = &action {
                    if get_table(*i as usize) != 0 {
                        println!("Invalid position! Try again.");
                    } else {
                        set_table(*i as usize, player);
                        available_tiles -= 1;
                        state = State::AiTurn;
                    }
                }
            },
            AiTurn => {
                println!("Ai playing its turn.");
                let mut available: Vec<usize> = vec![];
                for i in 0..9 {
                    if get_table(i) == 0 {
                        available.push(i);
                    }
                }
                let tile = rnd.gen_range(0..(available.len()));
                set_table(available[tile], ai);
                available_tiles -= 1;
                state = State::PlayerTurn;
            },
            GameOver(condition) => {
                match condition {
                    Tie => {
                        println!("{}", ansi_term::Colour::Yellow
                            .bold()
                            .paint("Game Over! It's a tie.")
                        );
                    },
                    Player => {
                        println!("{}", ansi_term::Colour::Green
                            .bold()
                            .paint("Game Over! Player won!")
                        );
                    },
                    Ai => {
                        println!("{}", ansi_term::Colour::Red
                            .bold()
                            .paint("Game Over! AI won!")
                        );
                    },
                }

                println!("Play Again? [Yes/No]");

                let action = get_action();
                match action {
                    Quit => {set_running(false)},
                    Yes => {
                        for i in 0..9 {
                            set_table(i, 0);
                        }
                        (player, ai) = pick_tokens(&mut rnd);
                        available_tiles = 9;
                        state = if pick_starter(&mut rnd) == player {
                            State::PlayerTurn
                        } else {
                            State::AiTurn
                        };
                    },
                    No => {set_running(false)},
                    _ => {}
                }
            }
        }

        if available_tiles == 0 {
            state = State::GameOver(Tie);
        }

        if let Quit = &action {
            set_running(false);
        }
    }
}
