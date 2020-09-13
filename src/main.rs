extern crate inkwell;

use std::fs;
use umbrella::compiler::compile;
use umbrella::lexer::lexers;
use umbrella::parser::parsers;

fn main() {
    let content = fs::read_to_string("./example/example.um").unwrap();
    let lex_result = lexers::run(&content);
    let mut parser = parsers::Persers::new(lex_result);
    let root = parser.run();
    match root {
        Ok(root) => {
            println!("{:?}", root);
            compile::compile(root);
        }
        Err(s) => {
            eprintln!("{}", s);
        }
    }
}
