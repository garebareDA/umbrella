use super::super::parser::ast;
use super::compile::CodeGen;

use inkwell::values;

impl<'ctx> CodeGen<'ctx> {
  pub fn call_write(&self, call: &ast::CallAST) {
    let function = self.module.get_function(&call.callee);
    match function {
      Some(func) => {
        let argument = self.argument_get(&call.argument);
        self.builder.build_call(
          func,
          &argument,
          "return",
        );
      }
      None => {

      }
    }
  }

  fn argument_get(&self, arguments: &Vec<ast::Types>) ->  Vec<values::BasicValueEnum>{
    let i32_type = self.context.i32_type();
    let mut argument_vec: Vec<values::BasicValueEnum> = Vec::new();
    for argument in arguments.iter() {
      match argument {
        ast::Types::Variable(var) => {
          match self.vars_serch(&var.name) {
            Ok(var) => match self.change_value(var.clone()) {
              Ok(value) => {
                argument_vec.push(value.into());
              }
              Err(()) => {
                //error
              }
            },
            Err(()) => {
              //error
            }
          }
        }

        ast::Types::Number(num) => {
          argument_vec.push(i32_type.const_int(num.num as u64, false).into());
        }

        ast::Types::Binary(bin) => {
          let result = self.calcuration(bin);
          argument_vec.push(result.into());
        }
        _ => {}
      }
    }

    return argument_vec;
  }
}