use crate::scanner::scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self};
use std::process;
mod error_handler;
mod token;
mod scanner;
mod grammar;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Wrong number of arguments");
        process::exit(64);
    }
    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Couldn't read file");
    run(&contents);

    if error_handler::had_error() {
        process::exit(65);
    }
}

fn run_prompt() -> io::Result<()> {
    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let mut buffer = String::new();
        let result = io::stdin().read_line(&mut buffer)?;
        println!("Buffer {}", buffer);
        if result == 0 {
            return Ok(());
        } else if buffer.eq("wot\n") {
            error_handler::error(0, "nope");
        } else {
            run(&buffer);
        }
        error_handler::set_error(false);
    }
}

fn run(input: &str) {
    let mut scanner = Scanner::new(input.to_string());
    
    for token in scanner.scan_tokens() {
        println!("{}", token);
    }
}
