#![warn(missing_debug_implementations)]

use crate::eval::interpreted_eval::EvalError;
use crate::eval::interpreted_eval::Eval;
use crate::parser::parser::Parser;
use crate::scanner::scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self};
use std::process;
mod error_handler;
mod token;
mod scanner;
mod grammar;
mod parser;
mod eval;
mod bytecode;
mod errors;

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
    } else if error_handler::had_runtime_error() {
        process::exit(70);
    }
}

fn run_prompt() -> io::Result<()> {
    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let mut buffer = String::new();
        let result = io::stdin().read_line(&mut buffer)?;
        if result == 0 {
            return Ok(());
        } else if buffer.eq("wot\n") {
            error_handler::error(0, "nope");
        } else {
            run(&buffer);
        }
        error_handler::set_error(false);
        error_handler::set_runtime_error(false);
    }
}

fn run(input: &str) {
    let mut scanner = Scanner::new(input.to_string());
    let (tokens, _errors) = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let parse_res = parser.parse().or_else(|error| Err(error.into()));

    match parse_res.and_then( |res| res.eval()) {
       Ok(eval_result) => println!("{}", eval_result),
       Err(EvalError{line, message}) => error_handler::runtime_error(line, &message)
    }
}
