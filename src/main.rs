mod scanner;
use scanner::tokenize;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(_contents: &str) -> Result<(), String> {
    let tokens = tokenize(_contents);
    for token in tokens {
        println!("{:?}", token)
    }
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        //What does flushing the buffer do
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Error flushing buff".to_string()),
        }

        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                dbg!(n);

                match run(&buffer) {
                    Ok(res) => (),
                    Err(msg) => {
                        println!("{}", msg);
                    }
                }

                if n <= 1 {
                    return Ok(());
                }
            }
            Err(_) => return Err("Couldn't read string".to_string()),
        }
        println!("You wrote {}", buffer);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(e) => {
                println!("Error {}", e);
            }
        }
    } else if args.len() == 1 {
        //repl
        match run_prompt() {
            Ok(_) => exit(0),
            Err(e) => {
                println!("Error {}", e);
                exit(1);
            }
        }
    } else {
        //Throw
        exit(64);
    }
    //let src = "abc_123 = 3
    //           print(hi)";
    //println!("tokens {:?}", tokenize(src));
}
