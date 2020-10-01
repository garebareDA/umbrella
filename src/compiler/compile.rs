use super::super::parser::ast;
use super::functions::Function;
use super::vars::Var;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values;
use inkwell::AddressSpace;

pub struct CodeGen<'ctx> {
  pub context: &'ctx Context,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
  pub var_vec: Vec<Vec<Var<'ctx>>>,
  pub function_vec: Vec<Vec<Function>>,
  pub if_gen_stack: usize,
  pub for_gen_stack: usize,
}

pub fn compile(ast: ast::RootAST) {
  let context = Context::create();
  let module = context.create_module("main");
  let builder = context.create_builder();

  let mut code_gen = CodeGen {
    context: &context,
    module,
    builder,
    var_vec: Vec::new(),
    function_vec: Vec::new(),
    if_gen_stack: 0,
    for_gen_stack: 0,
  };
  code_gen.add_fun_printf();
  match code_gen.set_main_run(&ast.node) {
    Ok(_) => {
      code_gen.set_return();
    }

    Err(s) => {
      eprintln!("{}", s);
    }
  }
}

impl<'ctx> CodeGen<'ctx> {
  pub fn judge(
    &mut self,
    types: &ast::Types,
    basic_block: inkwell::basic_block::BasicBlock,
  ) -> Result<(), String> {
    match types {
      ast::Types::Call(call) => {
        if call.callee == "print" && call.argument.len() < 2 {
          match &call.argument[0] {
            ast::Types::Strings(strings) => {
              let format = self
                .builder
                .build_global_string_ptr(&format!("{}\n", strings.strings), "strings");
              self.print(values::BasicValueEnum::PointerValue(
                format.as_pointer_value(),
              ));
            }

            ast::Types::Number(num) => {
              let i32_type = self.context.i32_type();
              let i32_const = i32_type.const_int(num.num as u64, false);
              self.print(values::BasicValueEnum::IntValue(i32_const));
            }

            ast::Types::Binary(bin) => {
              let sum = self.calcuration(bin);
              match sum {
                Ok(sum) => {
                  self.print(values::BasicValueEnum::IntValue(sum));
                }

                Err(s) => {
                  return Err(s);
                }
              }
            }

            ast::Types::Variable(var) => match self.vars_serch(&var.name) {
              Ok(var) => {
                self.print(*var);
              }
              Err(s) => {
                return Err(s);
              }
            },
            _ => return Err("print argment error".to_string()),
          }
          return Ok(());
        }

        let call = self.call_write(call);
        match call {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }
      }

      ast::Types::Ifs(ifs) => {
        let ifs = self.if_write(&ifs, basic_block);
        match ifs {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }
      }

      ast::Types::Fors(fors) => {
        let fors = self.for_write(&fors, basic_block);
        match fors {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }
      }

      ast::Types::Variable(var) => {
        if !var.node.is_empty() {
          let var = self.var_write(&var.name, &var.node[0]);
          match var {
            Ok(_) => {}
            Err(s) => {
              return Err(s);
            }
          }
        }
      }

      ast::Types::Return(ret) => {
        let returns = self.return_write(&ret.node[0]);
        match returns {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }
      }

      _ => {}
    }

    return Ok(());
  }

  pub fn scope_write(
    &mut self,
    node: &Vec<ast::Types>,
    basic_block: inkwell::basic_block::BasicBlock,
  ) -> Result<(), String> {
    for ast in node.iter() {
      self.builder.position_at_end(basic_block);
      match self.judge(ast, basic_block) {
        Ok(_) => {}
        Err(s) => {
          return Err(s);
        }
      }
    }
    return Ok(());
  }

  fn start_function_write(
    &mut self,
    node: &Vec<ast::Types>,
    basic_block: inkwell::basic_block::BasicBlock,
  ) -> Result<(), String> {
    for ast in node.iter() {
      self.builder.position_at_end(basic_block);
      match ast {
        ast::Types::Function(fun) => {
          self.push_fun(fun, &fun.name, &fun.param);
          self.push_var_vec();
          self.push_fun_vec();
          let fun = self.function_write(&fun);
          match fun {
            Ok(_) => {}
            Err(s) => {
              return Err(s);
            }
          }
          self.push_var_vec_remove();
          self.push_fun_vec_remove();
        }
        _ => {}
      }
    }

    return Ok(());
  }

  fn set_main_run(&mut self, node: &Vec<ast::Types>) -> Result<(), String> {
    self.push_var_vec();
    self.push_fun_vec();
    let i32_type = self.context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let function = self.module.add_function("main", main_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");
    match self.start_function_write(node, basic_block) {
      Ok(_) => {}
      Err(s) => {
        return Err(s);
      }
    }
    match self.scope_write(node, basic_block) {
      Ok(_) => {}
      Err(s) => {
        return Err(s);
      }
    }
    self.builder.position_at_end(basic_block);
    return Ok(());
  }

  pub fn add_fun_printf(&mut self) {
    let i32_type = self.context.i32_type();
    let i8_type = self.context.i8_type();
    let printf_type = i32_type.fn_type(&[i8_type.ptr_type(AddressSpace::Generic).into()], true);
    self.module.add_function("printf", printf_type, None);
  }

  fn set_return(&mut self) {
    let i32_type = self.context.i32_type();
    self
      .builder
      .build_return(Some(&i32_type.const_int(0, false)));
    self.module.print_to_stderr();
    self.module.print_to_file("./build/test.ll").expect("faild");
  }
}
