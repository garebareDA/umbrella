use super::super::parser::ast;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::OptimizationLevel;

pub struct CodeGen<'ctx> {
  pub context: &'ctx Context,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
  pub code_gen_stack: usize,
}

pub fn jit_compile(ast: ast::RootAST) {
  let context = Context::create();
  let module = context.create_module("main");
  let builder = context.create_builder();

  let mut code_gen = CodeGen {
    context: &context,
    module,
    builder,
    code_gen_stack: 0,
  };
  code_gen.add_fun_print();

  for ast in ast.node.iter() {
    code_gen.set_functions(ast);
  }

  code_gen.set_main();
  for ast in ast.node.iter() {
    code_gen.judge(ast);
  }
  code_gen.set_return();
}

impl<'ctx> CodeGen<'ctx> {
  pub fn set_functions(&mut self, types: &ast::Types) {
    match types {
      ast::Types::Ifs(ifs) => {
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("ifs", main_type, None);
        let basic_block_entry = self.context.append_basic_block(function, "entry");

        match &ifs.ifs[0] {
          ast::Types::Binary(bin) => {
            let basic_block_then = self.context.append_basic_block(function, "then");
            self.builder.position_at_end(basic_block_then);

            for ast in ifs.then.iter() {
              self.judge(&ast);
            }

            let basic_block_else = self.context.append_basic_block(function, "else");
            self.builder.position_at_end(basic_block_else);

            for ast in &ifs.elses {
              self.judge(&ast);
            }

            self.builder.position_at_end(basic_block_entry);
            let sum = self.calcuration(&bin);
            self
              .builder
              .build_conditional_branch(sum, basic_block_then, basic_block_else);

            let basic_block_end = self.context.append_basic_block(function, "end");
            self.builder.position_at_end(basic_block_end);
            self
              .builder
              .build_return(Some(&i32_type.const_int(0, false)));

            self.builder.position_at_end(basic_block_then);
            self.builder.build_unconditional_branch(basic_block_end);

            self.builder.position_at_end(basic_block_else);
            self.builder.build_unconditional_branch(basic_block_end);
          }
          _ => {}
        }
      }

      ast::Types::Fors(fors) => {
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("fors", main_type, None);
        let basic_block_entry = self.context.append_basic_block(function, "entry");
        let basic_block_preloop = self.context.append_basic_block(function, "preloop");
        let basic_block_loop = self.context.append_basic_block(function, "loop");
        let basic_block_afterloop = self.context.append_basic_block(function, "afterloop");

        self.builder.position_at_end(basic_block_entry);
        self.builder.build_unconditional_branch(basic_block_preloop);
        self.builder.position_at_end(basic_block_preloop);
        let variable = self.builder.build_phi(i32_type, "i");
        variable.add_incoming(&[(&i32_type.const_int(1, false), basic_block_entry)]);
        
      }
      _ => {}
    }
  }

  pub fn judge(&mut self, types: &ast::Types) {
    match types {
      ast::Types::Call(call) => {
        if call.callee == "print" && call.argument.len() < 2 {
          match &call.argument[0] {
            ast::Types::Strings(strings) => {
              self.print_string(&strings.strings);
            }

            ast::Types::Number(num) => {
              self.print_string(&num.num.to_string());
            }

            ast::Types::Binary(bin) => {
              let sum = self.calcuration(bin);
              self.print_string(&sum.print_to_string().to_string());
            }
            _ => {}
          }
        }
      }

      ast::Types::Ifs(_) => {
        if self.code_gen_stack == 0 {
          let ifs = self.module.get_function("ifs");
          self.builder.build_call(ifs.unwrap(), &[], "ifs");
          self.code_gen_stack += 1;
          return
        }

        let ifs = self.module.get_function(&format!("{}.{}","ifs", self.code_gen_stack));
        self.builder.build_call(ifs.unwrap(), &[], "ifs");
        self.code_gen_stack += 1;
      }

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

  pub fn add_fun_print(&mut self) {
    let i32_type = self.context.i32_type();
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    self.module.add_function("putchar", putchar_type, None);
  }

  fn set_return(&mut self) {
    let i32_type = self.context.i32_type();
    self
      .builder
      .build_return(Some(&i32_type.const_int(0, false)));
    self.module.print_to_stderr();
    self
      .module
      .print_to_file("./build/hello.ll")
      .expect("faild");
  }
}
