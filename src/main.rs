extern crate inkwell;

use std::env;
use std::fs;
use umbrella::compiler::compile;
use umbrella::lexer::lexers;
use umbrella::parser::parsers;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 && args[1] == "run" {
        let content = fs::read_to_string(&args[2]).unwrap();
        let lex_result = lexers::run(&content);
        let mut parser = parsers::Persers::new(lex_result);
        let root = parser.run();
        match root {
            Ok(root) => {
                println!("{:?}", root);
                compile::compile(root, &args[3]);
            }
            Err(s) => {
                eprintln!("{}", s);
            }
        }
    }else {
        println!("run [input path] [output path]");
    }
}
