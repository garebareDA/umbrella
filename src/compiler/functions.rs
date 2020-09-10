use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::types;
use inkwell::values;
use inkwell::AddressSpace;

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
  pub fn function_write(&mut self, funs: &ast::FunctionAST) {
    let (fn_type, name_vec) = self.function_param(&funs.param, &funs.returns);
    let function = self.module.add_function(&funs.name, fn_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");
    self.builder.position_at_end(basic_block);
    let function_param = function.get_params();
    for (index, param) in function_param.iter().enumerate() {
      let name = &name_vec[index];
      let value = values::BasicValueEnum::IntValue(param.into_int_value());
      self.push_var(value, name);
    }
    self.scope_write(&funs.node, basic_block);
    let i32_type = self.context.i32_type();
    self
      .builder
      .build_return(Some(&i32_type.const_int(0, false)));
  }

  fn function_param(
    &self,
    params: &Vec<ast::Types>,
    return_type: &Option<ast::VariableType>,
  ) -> (types::FunctionType<'ctx>, Vec<String>) {
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

          _ => {}
        },
        _ => {
          //error
        }
      }
    }

    match return_type {
      Some(t) => match t {
        ast::VariableType::Int => return (i32_type.fn_type(&param_vec, false), name_vec),

        ast::VariableType::Bool => {
          return (bool_type.fn_type(&param_vec, false), name_vec);
        }

        ast::VariableType::Strings => {
          return (
            i8_type
              .ptr_type(AddressSpace::Generic)
              .fn_type(&param_vec, false),
            name_vec,
          );
        }
      },

      None => {
        return (i32_type.fn_type(&param_vec, false), name_vec);
      }
    }
  }

  pub fn push_fun_vec(&mut self) {
    self.function_vec.push(Vec::new());
  }

  pub fn push_fun_vec_remove(&mut self) {
    self.function_vec.remove(self.function_vec.len() - 1);
  }

  pub fn push_fun(&mut self, value: &ast::FunctionAST, name: &str) {
    let fun_value = Function::new(name, value);
    let len = self.function_vec.len() - 1;
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
