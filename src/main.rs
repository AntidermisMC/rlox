use std::env;
use std::io::{Read, Write};

use crate::ast::statements::StatementVisitor;

use crate::scanning::TokenStream;

mod ast;
mod code_span;
mod error;
mod eval;
mod location;
mod location_tracking_iterator;
mod parsing;
mod scanning;

fn main() {
    let args: Vec<String> = env::args().collect();
    let res = match args.len() {
        0 => print_usage(),
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => print_usage(),
    };
    std::process::exit(res.unwrap_or(64) as i32)
}

fn print_usage() -> std::io::Result<u8> {
    eprintln!("Usage: rlox FILE");
    Ok(64)
}

/// Prompts the user to write code and processes it.
fn run_prompt() -> std::io::Result<u8> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input == "" {
            return Ok(0);
        }
        run(&mut input);
    }
}

/// Runs a whole file.
fn run_file(file_name: &str) -> std::io::Result<u8> {
    let mut file = std::fs::File::open(file_name)?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;
    run(&mut code);
    Ok(0)
}

/// Runs a single line of code.
fn run(code: &mut str) -> Option<u8> {
    let mut tokens = TokenStream::new(code);
    let tree = parsing::parse(&mut tokens);
    match tree {
        Err(e) => print!("{}", e),
        Ok(stmts) => {
            for stmt in stmts {
                let res = eval::Evaluator {}.visit_statement(&stmt);
                match res {
                    Ok(_) => (),
                    Err(e) => print!("{}", e),
                }
            }
        }
    }
    None
}
