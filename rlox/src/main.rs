use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read, Result, stdin};

static mut HAD_ERROR: bool = false;

struct LoxState {
    pub had_error: bool,
}

impl LoxState {
    pub fn new() -> Self {
        LoxState { had_error: false }
    }
}

fn main() {
    let mut args = args();
    let mut state = LoxState::new();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(0x01000000)
    } else if args.len() == 2 {
        match run_file(&mut state, &mut args.nth(1)) {
            Ok(()) => std::process::exit(0x0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0x01000000)
            }
        }
    } else {
        match run_prompt(&mut state) {
            Ok(()) => std::process::exit(0x0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0x01000000)
            }
        }
    }
}

fn run_file(state: &mut LoxState, path: &mut Option<String>) -> Result<()> {
    match path {
        Some(p) => {
            let file = File::open(p)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            run(state, contents);

            if state.had_error {
                std::process::exit(0x01000001)
            }

            Ok(())
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid path,\nUsage: rlox [script]",
        )),
    }
}

fn run_prompt(state: &mut LoxState) -> Result<()> {
    use std::io::{self, Write};

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {
                run(state, line);
                state.had_error = false;
            }
            Err(_) => break,
        }
    }
    Ok(())
}

fn run(state: &mut LoxState, source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}

fn error(state: &mut LoxState, line: i32, message: String) {
    report(state, line, String::new(), message);
}

fn report(state: &mut LoxState, line: i32, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
    state.had_error = true;
}
