use std::env;
use std::process;
use std::fs;
use std::io::{self};


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Wrong number of arguments");
        process::exit(64);
    } if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_file(path: &str) {
   let contents = fs::read_to_string(path).expect("Couldn't read file");
    run(&contents);
}

fn run_prompt() -> io::Result<()> {
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

fn run(x: &str) {
    println!("Running: {}", x);
}
