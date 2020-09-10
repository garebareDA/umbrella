use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::values;

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
  pub fn var_write(&mut self, name: &str, value: &ast::Types) {
    match value {
      ast::Types::Number(num) => {
        let i32_type = self.context.i32_type();
        let const_int = i32_type.const_int(num.num as u64, false);
        self.push_var(values::BasicValueEnum::IntValue(const_int), name);
      }
      ast::Types::Binary(bin) => {
        let sum = self.calcuration(bin);
        self.push_var(values::BasicValueEnum::IntValue(sum), name);
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
        let returns = self.call_write(call).unwrap();
        self.push_var(returns.try_as_basic_value().left().unwrap(), name);
      }
      _ => {}
    }
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

  pub fn vars_serch(&self, name: &str) -> Result<&values::BasicValueEnum<'ctx>, ()> {
    for reverse in self.var_vec.iter().rev() {
      for vars in reverse.iter().rev() {
        if vars.get_name() == name {
          return Ok(&vars.value);
        }
      }
    }

    return Err(());
  }

  pub fn change_value(
    &self,
    value: &values::AnyValueEnum<'ctx>,
  ) -> Result<values::BasicValueEnum<'ctx>, ()> {
    match value {
      values::AnyValueEnum::IntValue(int) => Ok(values::BasicValueEnum::IntValue(int.clone())),
      values::AnyValueEnum::PhiValue(phi) => Ok(values::BasicValueEnum::IntValue(
        phi.as_basic_value().into_int_value(),
      )),
      values::AnyValueEnum::PointerValue(pointer) => Ok(values::BasicValueEnum::PointerValue(pointer.clone())),
      _ => Err(()),
    }
  }
}
