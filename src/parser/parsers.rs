extern crate lelex;

use super::ast;
use super::super::lexer::token;

static TOKEN: token::Token = token::Token::new();

pub struct Persers {
  tokens: Vec<lelex::tokens::Tokens>,
  index:usize,
}

impl Persers{
  pub fn new(tokens: Vec<lelex::tokens::Tokens>) -> Self {
    Self {
      tokens: tokens,
      index:0
    }
  }

  pub fn run(&mut self) -> ast::RootAST{
    let mut root = ast::RootAST::new();
    let len = self.tokens.len();
    loop{
      let result = self.judge();

      match result{
        Ok(r) => {
          root.node.push(r);
        }

        Err(()) => {}
      }

      self.index += 1;
      if len <= self.index{
        break;
      }
    }
    return root;
  }

  fn judge(&mut self)-> Result<ast::Types, ()> {
    let token = self.get_tokens(self.index).get_token();

    if token == TOKEN._variable {
      if self.get_tokens(self.index + 1).get_token() == TOKEN._paren_left{
        let value = self.get_tokens(self.index).get_value();
        let mut callee = ast::CallAST::new(value);
        self.index_add(2);
        let inner = self.judge();
        callee.argument.push(inner.expect("error!"));
        self.index_add(1);
        return Ok(ast::Types::Call(callee));
      }

      let value = self.get_tokens(self.index).get_value();
      return Ok(ast::Types::Variable(ast::VariableAST::new(value)));
    }

    if token == TOKEN._string {
      let value = self.get_tokens(self.index).get_value();
      let strings = ast::StringAST::new(value);
      return Ok(ast::Types::Strings(strings));
    }

    return Err(());
  }

  fn get_tokens(&mut self, num: usize) -> &lelex::tokens::Tokens{
    return &self.tokens[num]
  }

  fn index_add(&mut self, num:usize) {
    self.index += num;
  }
}