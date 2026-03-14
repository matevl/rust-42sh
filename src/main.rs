mod lexer;
mod parser;
mod ast;
mod executor;
mod builtins;
mod io_backend;

use std::env;
use std::process;
use std::io::{self, Write};
use crate::io_backend::{IoBackend, InputType};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::executor::Executor;
use crate::ast::CommandList;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_source;
    let interactive;
    
    // Argument parsing
    if args.len() > 1 {
        if args[1] == "-c" {
            if args.len() > 2 {
                input_source = InputType::String(args[2].clone());
                interactive = false;
            } else {
                eprintln!("rust-42sh: -c: option requires an argument");
                process::exit(2);
            }
        } else {
            input_source = InputType::File(args[1].clone());
            interactive = false;
        }
    } else {
        input_source = InputType::Stdin;
        interactive = unsafe { libc::isatty(libc::STDIN_FILENO) == 1 };
    }

    let backend = match IoBackend::new(input_source) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("rust-42sh: {}", e);
            process::exit(1);
        }
    };

    let lexer = Lexer::new(backend);
    let mut parser = Parser::new(lexer);
    let executor = Executor::new();

    loop {
        if interactive {
            print!("rust-42sh$ ");
            io::stdout().flush().ok();
        }

        if interactive {
            match parser.parse_next_command() {
                Ok(Some(cmd)) => {
                    let cmd_list = CommandList {
                        commands: vec![cmd],
                        asynchronous: false,
                    };
                    if let Err(e) = executor.execute(&cmd_list) {
                        eprintln!("rust-42sh: {}", e);
                    }
                },
                Ok(None) => break, // EOF
                Err(e) => eprintln!("rust-42sh: syntax error: {}", e),
            }
        } else {
            match parser.parse_command_list() {
                 Ok(cmd_list) => {
                     if let Err(e) = executor.execute(&cmd_list) {
                         eprintln!("rust-42sh: {}", e);
                         process::exit(1);
                     }
                     break;
                 },
                 Err(e) => {
                     eprintln!("rust-42sh: syntax error: {}", e);
                     process::exit(2);
                 }
            }
        }
    }
}
