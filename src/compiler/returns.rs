use super::compile::CodeGen;

use super::super::parser::ast;

impl<'ctx> CodeGen<'ctx> {
  pub fn return_write(&self, node: &ast::Types) {
    match node {
      ast::Types::Strings(strings) => {
        let format = self
          .builder
          .build_global_string_ptr(&format!("{}\n", strings.strings), "strings");
        self.builder.build_return(Some(&format.as_pointer_value()));
      }
      ast::Types::Number(num) => {
        let i32_type = self.context.i32_type();
        self
          .builder
          .build_return(Some(&i32_type.const_int(num.num as u64, false)));
      }
      ast::Types::Binary(bin) => {
        let sum = self.calcuration(bin);
        self.builder.build_return(Some(&sum));
      }
      ast::Types::Variable(var) => match self.vars_serch(&var.name) {
        Ok(t) => match self.change_value(&t) {
          Ok(value) => {
            self.builder.build_return(Some(&value));
          }
          Err(()) => {}
        },
        Err(()) => {}
      },
      _ => {}
    }
  }
}
