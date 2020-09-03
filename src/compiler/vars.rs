use super::compile::CodeGen;
use inkwell::values;

pub struct Var <'ctx> {
  name:String,
  value:values::AnyValueEnum<'ctx>,
}

impl<'ctx> Var<'ctx> {
  pub fn new(name:&str, value:values::AnyValueEnum<'ctx>) -> Self {
    Self {
      name:name.to_string(),
      value: value,
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_value(self) -> values::AnyValueEnum<'ctx> {
    self.value
  }
}

impl<'ctx> CodeGen<'ctx> {
  pub fn push_var_vec(&mut self)   {
    self.var_vec.push(Vec::new());
  }

  pub fn push_var_vec_remove(&mut self) {
    self.var_vec.remove(0);
  }

  pub fn push_var(&mut self, value:values::AnyValueEnum<'ctx>, name:&str) {
    let var_value = Var::new(name, value);
    let len = self.var_vec.len() - 1;
    self.var_vec[len].push(var_value);
  }

  pub fn vars_serch(&self, name:&str) {
    
  }
}
