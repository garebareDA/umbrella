extern crate lelex;

use super::super::lexer::token;
use super::ast;

static TOKEN: token::Token = token::Token::new();

pub struct Persers {
  tokens: Vec<lelex::tokens::Tokens>,
  index: usize,
}

impl Persers {
  pub fn new(tokens: Vec<lelex::tokens::Tokens>) -> Self {
    Self {
      tokens: tokens,
      index: 0,
    }
  }

  pub fn run(&mut self) -> ast::RootAST {
    let mut root = ast::RootAST::new();
    let len = self.tokens.len();
    loop {
      let result = self.judge();

      match result {
        Ok(r) => {
          root.node.push(r);
        }
        Err(()) => {}
      }

      self.index_add(1);
      if len <= self.index {
        break;
      }
    }
    return root;
  }

  pub fn scope(&mut self) -> Vec<ast::Types> {
    let len = self.tokens.len();
    let mut types: Vec<ast::Types> = Vec::new();
    loop {
      let token = self.get_tokens(self.index).get_token();
      if token == TOKEN._braces_right {
        return types;
      }

      let result = self.judge();
      match result {
        Ok(r) => {
          types.push(r);
        }
        Err(()) => {}
      }

      self.index_add(1);
      if len <= self.index {
        return types;
      }
    }
  }

  fn judge(&mut self) -> Result<ast::Types, ()> {
    let token = self.get_tokens(self.index).get_token();
    let len = self.tokens.len();

    if token == TOKEN._variable {
      if len > 1 && self.get_tokens(self.index + 1).get_token() == TOKEN._paren_left {
        let value = self.get_tokens(self.index).get_value();
        let mut callee = ast::CallAST::new(value);

        self.index_add(2);

        let inner = self.judge();
        match inner {
          Ok(inner) => {
            callee.argument.push(inner);
          }

          Err(()) => {}
        }
        return Ok(ast::Types::Call(callee));
      }

      let value = self.get_tokens(self.index).get_value();
      return Ok(ast::Types::Variable(ast::VariableAST::new(value)));
    }

    if token == TOKEN._number {
      let value = self.get_tokens(self.index).get_value().parse().unwrap();
      let number = ast::NumberAST::new(value);
      let numbe_types = ast::Types::Number(number);
      let check = self.check_calc(&numbe_types);
      match check {
        Some(mut bin) => {
          let inner = self.judge_calc();
          match inner {
            Some(t) => {
              bin.node.push(t);
            }

            None => {}
          }
          return Ok(ast::Types::Binary(bin));
        }

        None => {
          return Ok(numbe_types);
        }
      }
    }

    if token == TOKEN._string {
      let value = self.get_tokens(self.index).get_value();
      let strings = ast::StringAST::new(value);
      return Ok(ast::Types::Strings(strings));
    }

    if token == TOKEN._if {
      self.index_add(1);
      match self.judge() {
        Ok(types) => {
          let mut ifs = ast::IfsAST::new();
          ifs.ifs.push(types);
          if self.get_tokens(self.index).get_token() == TOKEN._braces_left {
            self.index_add(1);
            ifs.then = self.scope();
          } else {
            //error
          }

          if self.get_tokens(self.index + 1).get_token() == TOKEN._else {
            self.index_add(2);
            if self.get_tokens(self.index).get_token() == TOKEN._braces_left {
              self.index_add(1);
              ifs.elses = self.scope();
            } else {
              //error
            }
          }

          return Ok(ast::Types::Ifs(ifs));
        }

        Err(()) => {
          //error
        }
      };
    }

    return Err(());
  }

  fn check_calc(&mut self, inner: &ast::Types) -> Option<ast::BinaryAST> {
    if self.tokens.len() > self.index + 1 {
      let token = self.get_tokens(self.index + 1).get_token();
      if token == TOKEN._add
        || token == TOKEN._sub
        || token == TOKEN._div
        || token == TOKEN._multi
        || token == TOKEN._greater
        || token == TOKEN._less
      {
        self.index_add(1);
        let value = self.get_tokens(self.index).get_value();
        let mut binary = ast::BinaryAST::new(value.chars().nth(0).unwrap());
        binary.node.push(inner.clone());
        return Some(binary);
      }
    }
    return None;
  }

  fn judge_calc(&mut self) -> Option<ast::Types> {
    let len = self.tokens.len();
    if len <= self.index + 1 {
      return None;
    }

    self.index_add(1);
    let token = self.get_tokens(self.index).get_token();
    if token == TOKEN._add
      || token == TOKEN._sub
      || token == TOKEN._div
      || token == TOKEN._multi
      || token == TOKEN._greater
      || token == TOKEN._less
    {
      let mut ast_bin = ast::BinaryAST::new(
        self
          .get_tokens(self.index)
          .get_value()
          .chars()
          .nth(0)
          .unwrap(),
      );

      match self.judge_calc() {
        Some(t) => {
          ast_bin.node.push(t);
        }

        None => {}
      }
      return Some(ast::Types::Binary(ast_bin));
    }

    if token == TOKEN._number {
      let mut ast_num =
        ast::NumberAST::new(self.get_tokens(self.index).get_value().parse().unwrap());
      match self.judge_calc() {
        Some(t) => {
          ast_num.node.push(t);
        }

        None => {}
      }
      return Some(ast::Types::Number(ast_num));
    }

    None
  }

  fn get_tokens(&mut self, num: usize) -> &lelex::tokens::Tokens {
    return &self.tokens[num];
  }

  fn index_add(&mut self, num: usize) {
    self.index += num;
  }
}

#[cfg(test)]
mod tests {
  use super::super::super::lexer::lexers;
  use super::super::parsers;
  use super::ast;
  #[test]
  fn variable() {
    let lex_result = lexers::run("test");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run();
    match result.node[0] {
      ast::Types::Variable(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn strings() {
    let lex_result = lexers::run("\"test\"");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run();
    match result.node[0] {
      ast::Types::Strings(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn callee() {
    let lex_result = lexers::run("test()");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run();
    match result.node[0] {
      ast::Types::Call(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn number() {
    let lex_result = lexers::run("1111");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run();
    match result.node[0] {
      ast::Types::Number(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn calu() {
    let lex_result = lexers::run("1 + 1 + 1 + 1");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run();
    match result.node[0] {
      ast::Types::Binary(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]

  fn ifs() {
    let lex_result = lexers::run(
      "if 1 > 0 {
      print(\"hello\");
    }else {
      print(\"world\");
    }",
    );
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run();
    match result.node[0] {
      ast::Types::Ifs(_) => {}
      _ => panic!("not"),
    }
  }
}
