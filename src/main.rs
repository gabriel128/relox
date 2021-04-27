use std::env;
use std::fs;
use std::io::{self};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

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

    if HAD_ERROR.load(Ordering::Relaxed) {
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
            error(0, "nope");
        } else {
            run(&buffer);
        }
        HAD_ERROR.store(false, Ordering::Relaxed);
    }
}

fn run(x: &str) {
    println!("Running: {}", x);
}

fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, where_it_was: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_it_was, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}
