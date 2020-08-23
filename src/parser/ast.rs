#[derive(Debug, Clone)]
pub enum Types {
  Call(CallAST),
  Strings(StringAST),
  Number(NumberAST),
  Binary(BinaryAST),
  Variable(VariableAST),
}

#[derive(Debug)]
pub struct RootAST {
  pub node: Vec<Types>,
}

impl RootAST {
  pub fn new() -> Self {
    RootAST { node: Vec::new() }
  }
}
#[derive(Debug, Clone)]
pub struct CallAST {
  pub callee: String,
  pub argument: Vec<Types>,
}

impl CallAST{
  pub fn new(callee:&str) -> Self{
    CallAST{
      callee:callee.to_string(),
      argument:Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct StringAST{
  pub strings:String,
  pub node: Vec<Types>,
}

impl StringAST{
  pub fn new(strings:&str) -> Self {
    StringAST{
      strings:strings.to_string(),
      node:Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct NumberAST {
  pub num: i64,
  pub node: Vec<Types>,
}

impl NumberAST {
  pub fn new(num:i64) -> Self {
    Self{
      num,
      node: Vec::new()
    }
  }
}

#[derive(Debug, Clone)]
pub struct BinaryAST {
  pub op: char,
  pub node: Vec<Types>,
}

impl BinaryAST {
  pub fn new(op:char) -> Self {
    Self{
      op,
      node:Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct VariableAST{
  pub name:String,
  pub node: Vec<Types>
}

impl VariableAST{
  pub fn new(name:&str) -> Self {
    VariableAST{
      name:name.to_string(),
      node:Vec::new()
    }
  }
}