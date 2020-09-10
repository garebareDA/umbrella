extern crate lelex;

use super::token;

static TOKEN: token::Token = token::Token::new();

pub fn run(word: &str) -> Vec<lelex::tokens::Tokens> {
  let mut lex = lelex::lexers::Lexer::new(word);
  lex.push_reserved_word(TOKEN._let, "let").unwrap();
  lex.push_reserved_word(TOKEN._if, "if").unwrap();
  lex.push_reserved_word(TOKEN._else, "else").unwrap();
  lex.push_reserved_word(TOKEN._for, "for").unwrap();
  lex.push_reserved_word(TOKEN._fn, "fn").unwrap();
  lex.push_reserved_word(TOKEN._return, "return").unwrap();
  lex.push_between_ward(TOKEN._string, "\"").unwrap();
  lex.set_other_token(TOKEN._variable).unwrap();
  lex.set_number_token(TOKEN._number).unwrap();

  let result = lex.run();
  let tokens = result.get_tokens();
  println!("{:?}", tokens);
  return tokens.clone();
}