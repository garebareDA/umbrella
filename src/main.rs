use umbrella::lexer::lexers;

use std::fs;

fn main() {
    let content = fs::read_to_string("./example/example.um").unwrap();
    lexers::run(&content);
}