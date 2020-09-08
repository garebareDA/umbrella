use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::types;

pub struct Function {
  name: String,
  value: ast::FunctionAST,
}

impl Function {
  pub fn new(name: &str, value: &ast::FunctionAST) -> Self {
    Self {
      name: name.to_string(),
      value: value.clone(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_value(&self) -> &ast::FunctionAST {
    &self.value
  }
}

impl<'ctx> CodeGen<'ctx> {
  pub fn function_write(&self, funs: ast::FunctionAST) {
    let fn_type = self.function_param(&funs.param);
    let function = self.module.add_function(&funs.name, fn_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");
    self.builder.position_at_end(basic_block);
    
  }

  fn function_param(&self, params: &Vec<ast::Types>) -> types::FunctionType<'ctx> {
    let i32_type = self.context.i32_type();
    let bool_type = self.context.bool_type();
    let mut param_vec: Vec<types::BasicTypeEnum> = Vec::new();
    let (param_type, _) = self.get_function_vec_param_type(params);

    for types in param_type.iter() {
      match types {
        ast::VariableType::Int => {
          param_vec.push(i32_type.into());
        }

        ast::VariableType::Bool => {
          param_vec.push(bool_type.into());
        }

        _ => {}
      }
    }

    return i32_type.fn_type(&param_vec, false);
  }

  fn get_function_vec_param_type(&self, params: &Vec<ast::Types>) -> (Vec<ast::VariableType>, Vec<String>) {
    let mut param_vec:Vec<ast::VariableType> = Vec::new();
    let mut name_vec:Vec<String> = Vec::new();
    for param in params.iter() {
      match param {
        ast::Types::Variable(var) => {
          match &var.types {
            Some(t) =>{
              param_vec.push(t.clone());
              name_vec.push(var.name.to_string());
            }

            _ =>{}
          }
        }
        _ => {
          //error
        }
      }
    }
    return (param_vec, name_vec);
  }

  pub fn push_fun_vec(&mut self) {
    self.function_vec.push(Vec::new());
  }

  pub fn push_fun_vec_remove(&mut self) {
    self.function_vec.remove(0);
  }

  pub fn push_fun(&mut self, value: &ast::FunctionAST, name: &str) {
    let fun_value = Function::new(name, value);
    let len = self.var_vec.len() - 1;
    self.function_vec[len].push(fun_value);
  }

  pub fn funs_serch(&self, name: &str) -> Result<&ast::FunctionAST, ()> {
    for reverse in self.function_vec.iter().rev() {
      for funs in reverse.iter().rev() {
        if funs.get_name() == name {
          return Ok(&funs.get_value());
        }
      }
    }

    return Err(());
  }
}
