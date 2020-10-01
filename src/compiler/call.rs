use super::super::parser::ast;
use super::compile::CodeGen;

use inkwell::values;

impl<'ctx> CodeGen<'ctx> {
  pub fn call_write(&self, call: &ast::CallAST) -> Result<values::CallSiteValue<'ctx>, String> {
    let serch = self.fucntions_serch(&call.callee);
    match serch {
      Ok(fun) => match self.type_inspection(fun.get_param(), &call.argument) {
        Ok(()) => {}
        Err(s) => {
          return Err(s);
        }
      },
      Err(s) => {
        return Err(s);
      }
    }

    let function = self.module.get_function(&call.callee);
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

  fn type_inspection(
    &self,
    params: &Vec<ast::Types>,
    arguments: &Vec<ast::Types>,
  ) -> Result<(), String> {
    if params.len() != arguments.len() {
      return Err(
        "
      The number of arguments is different"
          .to_string(),
      );
    }

    for (index, param) in params.iter().enumerate() {
      let argument = &arguments[index];
      match param {
        ast::Types::Variable(var) => match &var.types {
          Some(t) => match t {
            ast::VariableType::Bool => match argument {
              //TODO TypeにBooleanを追加したら実装
              _ => return Err(format!("{} Incorrect argument", var.name)),
            },
            ast::VariableType::Int => match argument {
              ast::Types::Number(_) => return Ok(()),
              _ => return Err(format!("{} Incorrect argument", var.name)),
            },
            ast::VariableType::Strings => match argument {
              ast::Types::Strings(_) => return Ok(()),
              _ => return Err(format!("{} Incorrect argument", var.name)),
            },
          },
          None => return Err(format!("{} Incorrect argument", var.name)),
        },
        _ => return Err(format!("Incorrect argument")),
      }
    }

    return Ok(());
  }
}
