mod frontend {
    pub mod ast;
    pub mod lexer;
    pub mod parser;
}
mod backend {
    pub mod interpreter;
}

use std::env;
use std::fs::File;
use std::io::{self, Read};

use frontend::lexer::Lexer;
use frontend::parser::Parser;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>.im", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse_all();

    let mut interpreter = backend::interpreter::Interpreter::new();
    interpreter.interpret_all(&ast);

    #[cfg(debug_assertions)]
    interpreter.print_context();
    Ok(())
}
