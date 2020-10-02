#[derive(Debug, Clone)]
pub enum Types {
  Call(CallAST),
  Strings(StringAST),
  Number(NumberAST),
  Binary(BinaryAST),
  Variable(VariableAST),
  Ifs(IfsAST),
  Fors(ForsAST),
  Return(ReturnAST),
  Function(FunctionAST),
  Vector(VectorAST),
}

#[derive(Debug, Clone)]
pub enum VariableType {
  Strings,
  Int,
  Bool,
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
  pub op: String,
  pub node: Vec<Types>,
}

impl BinaryAST {
  pub fn new(op:&str) -> Self {
    Self{
      op:op.to_string(),
      node:Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct VariableAST{
  pub name:String,
  pub types:Option<VariableType>,
  pub index:Option<i64>,
  pub node: Vec<Types>
}

impl VariableAST{
  pub fn new(name:&str) -> Self {
    VariableAST{
      name:name.to_string(),
      types:None,
      index:None,
      node:Vec::new()
    }
  }
}

#[derive(Debug, Clone)]
pub struct IfsAST{
  pub ifs:Vec<Types>,
  pub then: Vec<Types>,
  pub elses: Vec<Types>,
}

impl IfsAST {
  pub fn new() -> Self {
    Self{
      ifs:Vec::new(),
      then:Vec::new(),
      elses:Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ForsAST{
  pub init:Vec<Types>,
  pub ifs:Vec<Types>,
  pub count:Vec<Types>,
  pub node: Vec<Types>,
}

impl ForsAST {
  pub fn new() -> Self {
    Self{
      init:Vec::new(),
      ifs:Vec::new(),
      count:Vec::new(),
      node:Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ReturnAST {
  pub node: Vec<Types>,
}

impl ReturnAST {
  pub fn new() -> Self {
    Self {
      node: Vec::new()
    }
  }
}

#[derive(Debug, Clone)]
pub struct FunctionAST{
  pub name: String,
  pub param:Vec<Types>,
  pub returns:Option<VariableType>,
  pub node: Vec<Types>,
}

impl FunctionAST {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      returns:None,
      param:Vec::new(),
      node:Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct VectorAST {
  pub vec : Vec<Types>,
}

impl VectorAST {
  pub fn new() -> Self {
    Self {
      vec:Vec::new(),
    }
  }
}