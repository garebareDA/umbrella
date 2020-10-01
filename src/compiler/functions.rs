use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::types;
use inkwell::AddressSpace;

#[derive(Debug, Clone)]
pub struct Function {
  name: String,
  value: ast::FunctionAST,
  param:Vec<ast::Types>
}

impl Function {
  pub fn new(name: &str, value: &ast::FunctionAST, param: &Vec<ast::Types>) -> Self {
    Self {
      name: name.to_string(),
      value: value.clone(),
      param: param.clone(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_value(&self) -> &ast::FunctionAST {
    &self.value
  }

  pub fn get_param(&self) -> &Vec<ast::Types> {
    &self.param
  }
}

impl<'ctx> CodeGen<'ctx> {
  pub fn function_write(&mut self, funs: &ast::FunctionAST) -> Result<(), String> {
    let params = self.function_param(&funs.param, &funs.returns);
    match params {
      Ok((fn_type, name_vec)) => {
        let function = self.module.add_function(&funs.name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        let function_param = function.get_params();
        match self.push_var_param(function_param, &name_vec) {
          Ok(()) => {}
          Err(s) => {
            return Err(s);
          }
        }
        let scope = self.scope_write(&funs.node, basic_block);
        match scope {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }
        let i32_type = self.context.i32_type();
        self
          .builder
          .build_return(Some(&i32_type.const_int(0, false)));

        return Ok(());
      }
      Err(s) => {
        return Err(s);
      }
    }
  }

  fn function_param(
    &self,
    params: &Vec<ast::Types>,
    return_type: &Option<ast::VariableType>,
  ) -> Result<(types::FunctionType<'ctx>, Vec<String>), String> {
    let i32_type = self.context.i32_type();
    let bool_type = self.context.bool_type();
    let i8_type = self.context.i8_type();
    let mut param_vec: Vec<types::BasicTypeEnum> = Vec::new();
    let mut name_vec: Vec<String> = Vec::new();

    for param in params.iter() {
      match param {
        ast::Types::Variable(var) => match &var.types {
          Some(t) => {
            name_vec.push(var.name.to_string());
            match t {
              ast::VariableType::Int => {
                param_vec.push(i32_type.into());
              }

              ast::VariableType::Bool => {
                param_vec.push(bool_type.into());
              }

              ast::VariableType::Strings => {
                param_vec.push(i8_type.ptr_type(AddressSpace::Generic).into());
              }
            }
          }

          _ => {
            return Err("function param error".to_string());
          }
        },
        _ => {
          return Err("function param error".to_string());
        }
      }
    }

    match return_type {
      Some(t) => match t {
        ast::VariableType::Int => return Ok((i32_type.fn_type(&param_vec, false), name_vec)),

        ast::VariableType::Bool => {
          return Ok((bool_type.fn_type(&param_vec, false), name_vec));
        }

        ast::VariableType::Strings => {
          return Ok((
            i8_type
              .ptr_type(AddressSpace::Generic)
              .fn_type(&param_vec, false),
            name_vec,
          ));
        }
      },

      None => {
        return Ok((i32_type.fn_type(&param_vec, false), name_vec));
      }
    }
  }

  pub fn fucntions_serch(&self, name: &str) ->Result<&Function, String>  {
    for reverse in self.function_vec.iter().rev() {
      for function in reverse.iter().rev() {
        if function.get_name() == name {
          return Ok(function);
        }
      }
    }

    return Err(format!("{} Function not found", name));
  }

  pub fn push_fun_vec(&mut self) {
    self.function_vec.push(Vec::new());
  }

  pub fn push_fun_vec_remove(&mut self) {
    self.function_vec.remove(self.function_vec.len() - 1);
  }

  pub fn push_fun(&mut self, value: &ast::FunctionAST, name: &str, param: &Vec<ast::Types>) {
    let fun_value = Function::new(name, value, param);
    let len = self.function_vec.len() - 1;
    self.function_vec[len].push(fun_value);
  }
}
