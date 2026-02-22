use crate::scanner::Scanner;
use std::fs::File;
use std::io::{BufReader, Read, Result, stdin};

#[derive(Default)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(self, path: &mut Option<String>) -> Result<()> {
        match path {
            Some(p) => {
                let file = File::open(p)?;
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents)?;
                self.run(contents);

                if self.had_error {
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

    pub fn run_prompt(mut self) -> Result<()> {
        use std::io::{self, Write};

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            match stdin().read_line(&mut line) {
                Ok(_) => {
                    self.run(line);
                    self.had_error = false;
                }
                Err(_) => break,
            }
        }
        Ok(())
    }

    pub fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(self, line: i32, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn report(mut self, line: i32, location: String, message: String) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}
