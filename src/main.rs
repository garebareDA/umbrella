extern crate inkwell;

use umbrella::lexer::lexers;
use umbrella::parser::parsers;
use umbrella::compiler::compile;
use std::fs;

fn main() {
    let content = fs::read_to_string("./example/example.um").unwrap();
    let lex_result = lexers::run(&content);
    let mut parser = parsers::Persers::new(lex_result);
    let root = parser.run();
    println!("{:?}", root);
    compile::jit_compile(root);
}