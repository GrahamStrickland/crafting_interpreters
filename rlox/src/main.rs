use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read, Result, stdin};

fn main() {
    let mut args = args();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(0x01000000)
    } else if args.len() == 2 {
        match run_file(&mut args.nth(1)) {
            Ok(()) => std::process::exit(0x0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0x01000000)
            }
        }
    } else {
        match run_prompt() {
            Ok(()) => std::process::exit(0x0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0x01000000)
            }
        }
    }
}

fn run_file(path: &mut Option<String>) -> Result<()> {
    match path {
        Some(p) => {
            let file = File::open(p)?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            run(contents);
            Ok(())
        }
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid path,\nUsage: rlox [script]",
        )),
    }
}

fn run_prompt() -> Result<()> {
    use std::io::{self, Write};

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => run(input),
            Err(_) => break,
        }
    }
    Ok(())
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
