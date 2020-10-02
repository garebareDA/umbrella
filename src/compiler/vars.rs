use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::types;
use inkwell::values;
use inkwell::AddressSpace;

#[derive(Debug)]
pub struct Var<'ctx> {
  name: String,
  value: values::BasicValueEnum<'ctx>,
}

impl<'ctx> Var<'ctx> {
  pub fn new(name: &str, value: values::BasicValueEnum<'ctx>) -> Self {
    Self {
      name: name.to_string(),
      value: value,
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_value(self) -> values::BasicValueEnum<'ctx> {
    self.value
  }
}

impl<'ctx> CodeGen<'ctx> {
  pub fn var_write(
    &mut self,
    name: &str,
    value: &ast::Types,
    types: &Option<ast::VariableType>,
  ) -> Result<(), String> {
    match value {
      ast::Types::Number(num) => {
        let i32_type = self.context.i32_type();
        let const_int = i32_type.const_int(num.num as u64, false);
        self.push_var(values::BasicValueEnum::IntValue(const_int), name);
      }
      ast::Types::Binary(bin) => {
        let sum = self.calcuration(bin);
        match sum {
          Ok(sum) => {
            self.push_var(values::BasicValueEnum::IntValue(sum), name);
          }

          Err(s) => {
            return Err(s);
          }
        }
      }
      ast::Types::Strings(strings) => {
        let format = self
          .builder
          .build_global_string_ptr(&format!("{}\n", strings.strings), "strings");
        self.push_var(
          values::BasicValueEnum::PointerValue(format.as_pointer_value()),
          name,
        );
      }
      ast::Types::Call(call) => {
        let returns = self.call_write(call);
        match returns {
          Ok(returns) => {
            self.push_var(returns.try_as_basic_value().left().unwrap(), name);
          }

          Err(s) => {
            return Err(s);
          }
        }
      }
      ast::Types::Vector(vec) => match types {
        Some(t) => {
          let vec = self.vector_write(&vec, &t);
          match vec {
            Ok(vec) => {
              self.push_var(vec, name);
            }
            Err(s) => {
              return Err(s);
            }
          }
        }
        None => {
          return Err(format!("{} is vector type error", name));
        }
      },
      ast::Types::Variable(var) => {
        let serch = self.vars_serch(&var.name);
        let i32_type = self.context.i32_type();

        match serch {
          Ok(vars) => {
            match var.index {
              Some(num) => match vars {
                values::BasicValueEnum::VectorValue(vec) => {
                  let el = vec.const_extract_element(i32_type.const_int(num as u64, false));
                  self.push_var(el, name);
                  return Ok(());
                }
                _ => {
                  return Err(format!("Not a Vector"));
                }
              },
              None => {}
            }
            self.push_var(*vars, name);
          }
          Err(s) => {
            return Err(s);
          }
        }
      }

      _ => {
        return Err(format!("{} is type error", name));
      }
    }

    return Ok(());
  }

  pub fn push_var_vec(&mut self) {
    self.var_vec.push(Vec::new());
  }

  pub fn push_var_vec_remove(&mut self) {
    self.var_vec.remove(self.var_vec.len() - 1);
  }

  pub fn push_var(&mut self, value: values::BasicValueEnum<'ctx>, name: &str) {
    let var_value = Var::new(name, value);
    let len = self.var_vec.len() - 1;
    self.var_vec[len].push(var_value);
  }

  pub fn vars_serch(&self, name: &str) -> Result<&values::BasicValueEnum<'ctx>, String> {
    for reverse in self.var_vec.iter().rev() {
      for vars in reverse.iter().rev() {
        if vars.get_name() == name {
          return Ok(&vars.value);
        }
      }
    }

    return Err(format!("{} Variable not found", name));
  }

  pub fn vars_type(&self) -> (Vec<types::BasicTypeEnum<'ctx>>, Vec<String>) {
    let i32_type = self.context.i32_type();
    let i8_type = self.context.i8_type();
    let mut types: Vec<types::BasicTypeEnum<'ctx>> = Vec::new();
    let mut name_vec: Vec<String> = Vec::new();
    for reverse in self.var_vec.iter().rev() {
      for vars in reverse.iter().rev() {
        match vars.value {
          values::BasicValueEnum::IntValue(_) => types.push(i32_type.into()),
          values::BasicValueEnum::PointerValue(_) => {
            types.push(i8_type.ptr_type(AddressSpace::Generic).into())
          }
          values::BasicValueEnum::VectorValue(vec) => {
            types.push(i32_type.vec_type(vec.get_type().get_size()).into());
          }
          _ => {}
        }
        name_vec.push(vars.name.to_string());
      }
    }

    return (types, name_vec);
  }

  pub fn push_var_param(
    &mut self,
    function_param: Vec<values::BasicValueEnum<'ctx>>,
    name_vec: &Vec<String>,
  ) -> Result<(), String> {
    for (index, param) in function_param.iter().enumerate() {
      let name = &name_vec[index];
      match param {
        values::BasicValueEnum::IntValue(_) => {
          let value = values::BasicValueEnum::IntValue(param.into_int_value());
          self.push_var(value, name);
        }

        values::BasicValueEnum::PointerValue(_) => {
          let value = values::BasicValueEnum::PointerValue(param.into_pointer_value());
          self.push_var(value, name);
        }

        values::BasicValueEnum::VectorValue(_) => {
          let value = values::BasicValueEnum::VectorValue(param.into_vector_value());
          self.push_var(value, name);
        }

        _ => {
          return Err("Parament is incorrect".to_string());
        }
      }
    }
    return Ok(());
  }

  pub fn get_argment(
    &self,
    name_vec: &Vec<String>,
  ) -> Result<Vec<values::BasicValueEnum<'ctx>>, String> {
    let mut arguments: Vec<values::BasicValueEnum<'ctx>> = Vec::new();
    for name in name_vec.iter() {
      match self.vars_serch(name) {
        Ok(value) => {
          arguments.push(value.clone());
        }
        Err(s) => return Err(s),
      }
    }

    return Ok(arguments);
  }
}
