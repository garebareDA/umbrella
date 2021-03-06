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

  pub fn run(&mut self) -> Result<ast::RootAST, String> {
    let mut root = ast::RootAST::new();
    let len = self.tokens.len();
    loop {
      let token = self.get_tokens(self.index).get_token();
      if token == TOKEN._end {
        self.index_add(1);
        if len <= self.index {
          break;
        }
        continue;
      }

      let result = self.judge();
      match result {
        Ok(r) => {
          root.node.push(r);
        }
        Err(s) => {
          return Err(s);
        }
      }

      self.index_add(1);
      if len <= self.index {
        break;
      }
    }
    return Ok(root);
  }

  pub fn scope(&mut self) -> Result<Vec<ast::Types>, String> {
    let len = self.tokens.len();
    let mut types: Vec<ast::Types> = Vec::new();
    loop {
      let token = self.get_tokens(self.index).get_token();

      if token == TOKEN._end {
        self.index_add(1);
        continue;
      }

      if token == TOKEN._braces_right {
        return Ok(types);
      }

      let result = self.judge();
      match result {
        Ok(r) => {
          types.push(r);
        }
        Err(s) => {
          return Err(s);
        }
      }

      self.index_add(1);
      if len <= self.index {
        return Ok(types);
      }
    }
  }

  fn judge(&mut self) -> Result<ast::Types, String> {
    let token = self.get_tokens(self.index).get_token();
    let len = self.tokens.len();

    if token == TOKEN._variable {
      //関数の解析
      if len > 1 && self.get_tokens(self.index + 1).get_token() == TOKEN._paren_left {
        let value = self.get_tokens(self.index).get_value();
        let mut callee = ast::CallAST::new(value);
        self.index_add(2);

        loop {
          if self.get_tokens(self.index).get_token() == TOKEN._paren_right {
            break;
          }
          let inner = self.judge();
          match inner {
            Ok(inner) => {
              callee.argument.push(inner);
              if self.get_tokens(self.index).get_token() == TOKEN._paren_right {
                break;
              }
            }

            Err(s) => {
              return Err(s);
            }
          }
          self.index_add(1);
          if self.get_tokens(self.index).get_token() == TOKEN._paren_right {
            break;
          }
          self.index_add(1);
        }

        return Ok(ast::Types::Call(callee));
      }

      if len > 1 && self.get_tokens(self.index + 1).get_token() == TOKEN._square_brackets_left {
        let value = self.get_tokens(self.index).get_value();
        let mut variable_ast = ast::VariableAST::new(value);
        self.index_add(2);
        let judge = self.judge();
        match judge {
          Ok(t) => match t {
            ast::Types::Number(num) => {
              variable_ast.index = Some(num.num);
              self.index_add(1);
              if self.get_tokens(self.index).get_token() == TOKEN._square_brackets_right {
                return Ok(ast::Types::Variable(variable_ast));
              }else {
                return Err(format!("']' there is not"));
              }
            }
            _ => return Err(format!("Index is not int")),
          },
          Err(s) => {
            return Err(s);
          }
        }
      }

      let value = self.get_tokens(self.index).get_value();
      let mut variabel_ast = ast::VariableAST::new(value);
      if self.tokens.len() > self.index + 1
        && self.get_tokens(self.index + 1).get_token() == TOKEN._colon
      {
        self.index_add(2);
        variabel_ast.types = self.variable_type_get();
      }
      let variable_type = ast::Types::Variable(variabel_ast);
      let check = self.check_calc(&variable_type);
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
          return Ok(variable_type);
        }
      }
    }

    if token == TOKEN._let {
      let none_type = self.get_tokens(self.index + 2).get_token() == TOKEN._equal;
      let ok_type = self.get_tokens(self.index + 4).get_token() == TOKEN._equal;
      if none_type || ok_type {
        self.index_add(1);
        let vars = self.judge();
        match vars {
          Ok(var) => match var {
            ast::Types::Variable(mut vars) => {
              self.index_add(2);
              match self.judge() {
                Ok(t) => {
                  vars.node.push(t);
                }

                Err(s) => {
                  return Err(s);
                }
              }
              return Ok(ast::Types::Variable(vars));
            }
            _ => {}
          },
          Err(s) => {
            return Err(s);
          }
        }
      } else {
        if !none_type {
          let value = self.get_tokens(self.index + 2).get_value();
          return Err(format!("{} is parse error", value));
        } else if !ok_type {
          let value = self.get_tokens(self.index + 4).get_value();
          return Err(format!("{} is parse error", value));
        }
      }
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
            match self.scope() {
              Ok(node) => {
                ifs.then = node;
              }

              Err(e) => {
                return Err(e);
              }
            };
          } else {
            return Err(format!(
              " \"{}\" If statement parsing error",
              self.get_tokens(self.index).get_value()
            ));
          }

          if self.tokens.len() <= self.index + 1 {
            return Ok(ast::Types::Ifs(ifs));
          }

          if self.get_tokens(self.index + 1).get_token() == TOKEN._else {
            self.index_add(2);
            if self.get_tokens(self.index).get_token() == TOKEN._braces_left {
              self.index_add(1);
              match self.scope() {
                Ok(node) => {
                  ifs.elses = node;
                }
                Err(e) => {
                  return Err(e);
                }
              }
            } else {
              return Err(format!(
                " \"{}\" Else statement parsing error",
                self.get_tokens(self.index).get_value()
              ));
            }
          }

          return Ok(ast::Types::Ifs(ifs));
        }

        Err(s) => {
          return Err(s);
        }
      };
    }

    if token == TOKEN._for {
      self.index_add(1);
      let mut fors = ast::ForsAST::new();
      match self.judge() {
        Ok(t) => match t {
          ast::Types::Variable(_) => {
            fors.init.push(t);
          }
          _ => {
            return Err(format!(
              "{} Unable to initialize for",
              self.get_tokens(self.index).get_value()
            ));
          }
        },
        Err(s) => return Err(s),
      }
      self.index_add(2);

      match self.judge() {
        Ok(t) => match t {
          ast::Types::Binary(_) => {
            fors.ifs.push(t);
          }
          _ => {
            return Err(format!(
              "{}
              Termination condition error",
              self.get_tokens(self.index).get_value()
            ));
          }
        },
        Err(s) => {
          return Err(s);
        }
      }
      self.index_add(1);

      match self.judge() {
        Ok(t) => match t {
          ast::Types::Binary(_) => {
            fors.count.push(t);
          }
          _ => {
            return Err(format!(
              "{} Counter error",
              self.get_tokens(self.index).get_value()
            ));
          }
        },
        Err(s) => return Err(s),
      }
      self.index_add(1);

      match self.scope() {
        Ok(node) => {
          fors.node = node;
        }

        Err(s) => {
          return Err(s);
        }
      }
      return Ok(ast::Types::Fors(fors));
    }

    if token == TOKEN._fn {
      //error処理する際にリファクタリング
      self.index_add(1);
      if TOKEN._variable == self.get_tokens(self.index).get_token() {
        let mut function_ast = ast::FunctionAST::new(self.get_tokens(self.index).get_value());
        self.index_add(1);
        if self.get_tokens(self.index).get_token() == TOKEN._paren_left {
          loop {
            self.index_add(1);
            let token = self.get_tokens(self.index).get_token();
            if token == TOKEN._paren_right {
              self.index_add(1);
              break;
            }

            if token == TOKEN._comma {
              continue;
            }

            let param = self.judge();
            match param {
              Ok(t) => {
                function_ast.param.push(t);
              }
              Err(s) => {
                return Err(s);
              }
            }
          }

          if self.get_tokens(self.index).get_token() != TOKEN._braces_left {
            let types = self.variable_type_get();
            function_ast.returns = types;
            self.index_add(1);
          }

          if self.get_tokens(self.index).get_token() == TOKEN._braces_left {
            self.index_add(1);
            match self.scope() {
              Ok(node) => {
                function_ast.node = node;
              }

              Err(s) => {
                return Err(s);
              }
            }
          }

          return Ok(ast::Types::Function(function_ast));
        }
      }
    }

    //vectorの解析
    if token == TOKEN._square_brackets_left {
      let mut vec = ast::VectorAST::new();
      self.index_add(1);
      loop {
        let token = self.get_tokens(self.index).get_token();
        if token == TOKEN._square_brackets_right {
          self.index_add(1);
          break;
        }

        if token == TOKEN._comma {
          self.index_add(1);
        }

        match self.judge() {
          Ok(t) => {
            vec.vec.push(t);
          }
          Err(s) => {
            return Err(s);
          }
        }
        self.index_add(1);
      }
      return Ok(ast::Types::Vector(vec));
    }

    if token == TOKEN._return {
      self.index_add(1);
      match self.judge() {
        Ok(t) => match t {
          ast::Types::Ifs(_) => {}
          ast::Types::Fors(_) => {}
          ast::Types::Function(_) => {}
          _ => {
            let mut returns_ast = ast::ReturnAST::new();
            returns_ast.node.push(t);
            return Ok(ast::Types::Return(returns_ast));
          }
        },
        Err(s) => {
          return Err(s);
        }
      }
    }

    return Err(format!(
      "{} is parse error",
      self.get_tokens(self.index).get_value()
    ));
  }

  fn check_calc(&mut self, inner: &ast::Types) -> Option<ast::BinaryAST> {
    if self.tokens.len() > self.index + 1 {
      let token = self.get_tokens(self.index + 1).get_token();
      if token == TOKEN._add || token == TOKEN._sub || token == TOKEN._div || token == TOKEN._multi
      {
        self.index_add(1);
        let value = self.get_tokens(self.index).get_value();
        let mut binary = ast::BinaryAST::new(value);
        binary.node.push(inner.clone());
        return Some(binary);
      }

      if token == TOKEN._equal && self.get_tokens(self.index + 2).get_token() != TOKEN._equal {
        return None;
      }

      if token == TOKEN._greater
        || token == TOKEN._less
        || token == TOKEN._equal
        || token == TOKEN._exclamation
      {
        self.index_add(1);
        let token = self.get_tokens(self.index + 1).get_token();
        if token == TOKEN._equal {
          self.index_add(1);
          let value = self.get_tokens(self.index - 1).get_value();
          let next_value = &self.get_tokens(self.index).get_value();
          let value_op = format!("{}{}", value, next_value);
          let mut binary = ast::BinaryAST::new(&value_op);
          binary.node.push(inner.clone());
          return Some(binary);
        }

        let value = self.get_tokens(self.index).get_value();
        let mut binary = ast::BinaryAST::new(value);
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
    if token == TOKEN._add || token == TOKEN._sub || token == TOKEN._div || token == TOKEN._multi {
      let mut ast_bin = ast::BinaryAST::new(self.get_tokens(self.index).get_value());
      match self.judge_calc() {
        Some(t) => {
          ast_bin.node.push(t);
        }

        None => {}
      }
      return Some(ast::Types::Binary(ast_bin));
    }

    if token == TOKEN._greater
      || token == TOKEN._less
      || token == TOKEN._equal
      || token == TOKEN._exclamation
    {
      let token = self.get_tokens(self.index + 1).get_token();
      let value = self.get_tokens(self.index + 1).get_value();
      if token == TOKEN._equal {
        let mut ast_bin = ast::BinaryAST::new(&format!(
          "{}{}",
          self.get_tokens(self.index).get_value(),
          value
        ));
        self.index_add(1);
        match self.judge_calc() {
          Some(t) => {
            ast_bin.node.push(t);
          }
          None => {}
        }
        return Some(ast::Types::Binary(ast_bin));
      }

      let mut ast_bin = ast::BinaryAST::new(self.get_tokens(self.index).get_value());
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

  fn variable_type_get(&self) -> Option<ast::VariableType> {
    if self.get_tokens(self.index).get_token() == TOKEN._variable {
      let value = self.get_tokens(self.index).get_value();
      if value == "int" {
        return Some(ast::VariableType::Int);
      }

      if value == "string" {
        return Some(ast::VariableType::Strings);
      }

      if value == "bool" {
        return Some(ast::VariableType::Bool);
      }
    }
    None
  }

  fn get_tokens(&self, num: usize) -> &lelex::tokens::Tokens {
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
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Variable(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn strings() {
    let lex_result = lexers::run("\"test\"");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Strings(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn callee() {
    let lex_result = lexers::run("test(a)");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Call(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn number() {
    let lex_result = lexers::run("1111");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Number(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn calu() {
    let lex_result = lexers::run("1 + 1 + 1 + 1");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run().unwrap();
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
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Ifs(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn fors() {
    let lex_result = lexers::run(
      "for let i = 0; i < 5; i++ {
        print(\"hello\");
      }",
    );
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Fors(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn function() {
    let lex_result = lexers::run("fn a (a:int, b:string){}");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Function(_) => {}
      _ => panic!("not"),
    }
  }

  #[test]
  fn print() {
    let lex_result = lexers::run("print(\"a\")");
    let mut parser = parsers::Persers::new(lex_result);
    let result = parser.run().unwrap();
    match result.node[0] {
      ast::Types::Call(_) => {}
      _ => panic!("not"),
    }
  }
}
