use std::process::exit;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
        }
    }

    pub fn run_file(&mut self, path: String) {
        let source = std::fs::read_to_string(path).expect("Failed to read file");
        self.run(source);
        if self.had_error {
            exit(65);
        }
        if self.had_runtime_error {
            exit(70);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            if line.is_empty() {
                break;
            }
            self.run(line);
            self.had_error = false;
        }
    }

    pub fn run(&mut self, source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{}", token);
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    pub fn runtime_error(&mut self, error: RuntimeError) {
        self.report(error.token.line, &error.token.lexeme, error.message);
        self.had_runtime_error = true;
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}
// macro report!
macro_rules! report {
    ($line:expr, $message:expr) => {
        eprintln!("[line {}] Error: {}", $line, $message);
    };
}

fn error(line: usize, message: &str) {
    report!(line, message);
}

// Write a main function that get inputs from args
fn main() {
    // Get the first argument
    let mut args = std::env::args();
    if args.len() > 2 {
        exit(64);
    } else if args.len() == 2 {
        Lox::new().run_file(args.nth(1).unwrap())
    } else {
        Lox::new().run_prompt();
    }
}
