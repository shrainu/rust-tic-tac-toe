use std::io;
use std::io::{stdin, stdout, Write};

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

    return Ok(line);
}

fn get_number(msg : &str) -> Result<i32, String> {

    let line = get_line(msg);

    return match line {
        Ok(mut l) => {

            l.remove(l.len() - 1);

            if let Ok(i) = l.parse::<i32>() {
                Ok(i)
            } else {
                let err = format!("ERROR: Could not parse i32 from `{}`", l);
                Err(err)
            }
        },
        Err(e) => Err(e.to_string())
    }
}

fn main() {

    let num = get_number("Enter a number: ");

    match num {
        Ok(i) => { println!("{}^2 = {}", i, i * i); },
        Err(e) => { println!("{}", e); }
    }
}
