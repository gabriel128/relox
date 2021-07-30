#![warn(missing_debug_implementations)]

use crate::eval::interpreted_eval::Eval;
use crate::parser::parser::Parser;
use crate::scanner::scanner::Scanner;
use std::env;
use std::io;
use std::process;
mod token;
mod scanner;
mod grammar;
mod parser;
mod eval;
mod bytecode;
mod errors;


pub type Result<T, E = errors::ReloxError> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Wrong number of arguments");
        process::exit(64);
    }
    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_repl()?;
    }

    Ok(())
}

fn run_file(_path: &str) {
    // let contents = fs::read_to_string(path).expect("Couldn't read file");
    // run(&contents);

    // if compile error {
    //     process::exit(65);
    // } else if runtime error {
    //     process::exit(70);
    // }
}

fn run_repl() -> Result<()> {
    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let mut buffer = String::new();
        let result = io::stdin().read_line(&mut buffer)?;
        if result == 0 {
            return Ok(());
        } else {
            run(&buffer);
        }
    }
}

fn run(input: &str) {
    let scanner = Scanner::new(input.to_string());
    let tokens = match scanner.scan_tokens() {
       Ok(result) => result,
        Err(error) => {
            eprintln!("{}", error);
            Vec::new()
        }
    };

    let mut parser = Parser::new(tokens);
    let parse_res = parser.parse().or_else(|error| Err(error.into()));

    match parse_res.and_then( |res| res.eval()) {
       Ok(eval_result) => println!("{}", eval_result),
       Err(error) => eprintln!("{}", error)
    }
}
