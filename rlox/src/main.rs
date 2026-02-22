use rlox::lox::Lox;
use std::env::args;

fn main() {
    let mut args = args();
    let lox = Lox::new();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(0x01000000)
    } else if args.len() == 2 {
        match lox.run_file(&mut args.nth(1)) {
            Ok(()) => std::process::exit(0x0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0x01000000)
            }
        }
    } else {
        match lox.run_prompt() {
            Ok(()) => std::process::exit(0x0),
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(0x01000000)
            }
        }
    }
}
