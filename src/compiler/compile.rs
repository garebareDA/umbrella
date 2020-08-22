use super::super::parser::ast;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::OptimizationLevel;
use std::error::Error;

struct CodeGen<'ctx> {
  context: &'ctx Context,
  module: Module<'ctx>,
  builder: Builder<'ctx>,
}

pub fn jit_compile(ast: ast::RootAST) {
  let context = Context::create();
  let module = context.create_module("main");
  let builder = context.create_builder();

  let mut code_gen = CodeGen {
    context: &context,
    module,
    builder,
  };

  code_gen.set_main();
  code_gen.add_fun_print();
  for ast in ast.node.iter() {
    code_gen.judge(ast);
  }
  code_gen.set_return();
}

impl<'ctx> CodeGen<'ctx>  {
  pub fn add_fun_print(&mut self) {
    let i32_type = self.context.i32_type();
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    self.module.add_function("putchar", putchar_type, None);
  }

  pub fn judge(&mut self, types: &ast::Types) {
    match types {
      ast::Types::Call(call) => {
        if call.callee == "print" {
          match &call.argument[0] {
            ast::Types::Strings(strings) => {
              let i32_type = self.context.i32_type();
              let putchar = self.module.get_function("putchar");
              let word = strings.strings.to_string() + "\n";
              for c in word.chars() {
                let ascii = c.to_string().as_bytes()[0] as u64;
                self.builder.build_call(
                  putchar.unwrap(),
                  &[i32_type.const_int(ascii, false).into()],
                  "putchar",
                );
              }
            }
            _ => {}
          }
        }
      }

      ast::Types::Strings(string) => {}

      _ => {}
    }
  }

  fn set_main(&mut self) {
    let i32_type = self.context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let function = self.module.add_function("main", main_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");
    self.builder.position_at_end(basic_block);
  }

  fn set_return(&mut self) {
    let i32_type = self.context.i32_type();
    self
      .builder
      .build_return(Some(&i32_type.const_int(0, false)));
    self.module.print_to_stderr();
    self.module.print_to_file("./build/hello.ll").expect("faild");
  }
}
