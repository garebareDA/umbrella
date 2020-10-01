use super::super::parser::ast;
use super::compile::CodeGen;

use inkwell::values;

impl<'ctx> CodeGen<'ctx> {
  pub fn call_write(&self, call: &ast::CallAST) -> Result<values::CallSiteValue<'ctx>, String> {
    let function = self.module.get_function(&call.callee);
    let serch = self.fucntions_serch(&call.callee);

    match serch {
      Ok(_) => {},
      Err(s) => {
        return Err(s);
      }
    }

    match function {
      Some(func) => {
        let argument = self.argument_get(&call.argument);
        match argument {
          Ok(arg) => {
            let returns = self.builder.build_call(func, &arg, "return");
            return Ok(returns);
          }

          Err(s) => {
            return Err(s);
          }
        }
      }
      None => {
        return Err(format!("{} not found function", &call.callee));
      }
    }
  }

  fn argument_get(
    &self,
    arguments: &Vec<ast::Types>,
  ) -> Result<Vec<values::BasicValueEnum<'ctx>>, String> {
    let i32_type = self.context.i32_type();
    let mut argument_vec: Vec<values::BasicValueEnum> = Vec::new();
    for argument in arguments.iter() {
      match argument {
        ast::Types::Variable(var) => match self.vars_serch(&var.name) {
          Ok(var) => {
            argument_vec.push(*var);
          }
          Err(s) => {
            return Err(s);
          }
        },

        ast::Types::Number(num) => {
          argument_vec.push(i32_type.const_int(num.num as u64, false).into());
        }

        ast::Types::Binary(bin) => {
          let result = self.calcuration(bin);
          match result {
            Ok(result) => {
              argument_vec.push(result.into());
            }

            Err(s) => {
              return Err(s);
            }
          }
        }
        _ => {}
      }
    }

    return Ok(argument_vec);
  }
}
