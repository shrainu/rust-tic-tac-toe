use std::io;
use std::io::{stdin, stdout, Write};
use crate::Action::{Place, Quit, Unknown};

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

static TABLE: [i32;9] = [0; 9];
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
    } else if let Ok(i) = parse_number(&line) {
        return Place(i);
    } else {
        return Unknown;
    }
}

fn main() {

    set_running(true);

    while get_running() {

        match get_action() {
            Quit => {
                set_running(false);
            }
            Place(i) => {
                println!("Place : {}", i);
            }
            Unknown => {
                println!("Unknown action.");
            }
        }
    }
}
