extern crate lelex;

use super::token;

static TOKEN: token::Token = token::Token::new();

pub fn run(word:&str) -> Vec<lelex::tokens::Tokens>{
  let mut lex = lelex::lexers::Lexer::new(word);
  lex.push_reserved_word(TOKEN._let, "let").unwrap();
  lex.push_reserved_word(TOKEN._print, "print").unwrap();
  lex.set_other_token(TOKEN._variable).unwrap();
  lex.set_number_token(TOKEN._number).unwrap();

  let result = lex.run();
  let tokens = result.get_tokens();
  for token in tokens.iter(){
    println!("{} {}", token.get_value(), token.get_token());
  }
  return tokens.clone();
}