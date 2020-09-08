use super::super::parser::ast;
use super::functions::Function;
use super::vars::Var;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

pub struct CodeGen<'ctx> {
  pub context: &'ctx Context,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
  pub var_vec: Vec<Vec<Var<'ctx>>>,
  pub function_vec: Vec<Vec<Function>>,
  pub if_gen_stack: usize,
  pub for_gen_stack: usize,
}

pub fn jit_compile(ast: ast::RootAST) {
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
  code_gen.add_fun_print();
  code_gen.set_main_run(&ast.node);
  code_gen.set_return();
}

impl<'ctx> CodeGen<'ctx> {
  pub fn judge(&mut self, types: &ast::Types, basic_block: inkwell::basic_block::BasicBlock) {
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

      ast::Types::Ifs(ifs) => {
        self.push_var_vec();
        self.push_fun_vec();
        self.if_write(&ifs);
        self.builder.position_at_end(basic_block);
        if self.if_gen_stack == 0 {
          let ifs = self.module.get_function("ifs");
          self.builder.build_call(ifs.unwrap(), &[], "ifs");
          self.if_gen_stack += 1;
          self.push_var_vec_remove();
          self.push_fun_vec_remove();
          return;
        }

        let ifs = self
          .module
          .get_function(&format!("{}.{}", "ifs", self.if_gen_stack));
        self.builder.build_call(ifs.unwrap(), &[], "ifs");
        self.if_gen_stack += 1;
        self.push_var_vec_remove();
        self.push_fun_vec_remove();
      }

      ast::Types::Fors(fors) => {
        self.push_var_vec();
        self.push_fun_vec();
        self.for_write(&fors);
        self.builder.position_at_end(basic_block);
        if self.for_gen_stack == 0 {
          let ifs = self.module.get_function("fors");
          self.builder.build_call(ifs.unwrap(), &[], "fors");
          self.for_gen_stack += 1;
          self.push_var_vec_remove();
          self.push_fun_vec_remove();
          return;
        }

        let fors = self
          .module
          .get_function(&format!("{}.{}", "fors", self.for_gen_stack));
        self.builder.build_call(fors.unwrap(), &[], "fors");
        self.for_gen_stack += 1;
        self.push_var_vec_remove();
        self.push_fun_vec_remove();
      }

      ast::Types::Function(fun) => {
        self.push_fun(fun, &fun.name);
        self.push_var_vec();
        self.push_fun_vec();
        self.function_write(&fun);
        self.push_var_vec_remove();
        self.push_fun_vec_remove();
      }

      _ => {}
    }
  }

  pub fn scope_write(
    &mut self,
    node: &Vec<ast::Types>,
    basic_block: inkwell::basic_block::BasicBlock,
  ) {
    self.push_var_vec();
    for ast in node.iter() {
      self.builder.position_at_end(basic_block);
      self.judge(ast, basic_block);
    }
  }

  fn set_main_run(&mut self, node: &Vec<ast::Types>) {
    self.push_var_vec();
    self.push_fun_vec();
    let i32_type = self.context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let function = self.module.add_function("main", main_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");
    self.scope_write(node, basic_block);
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
    self.module.print_to_file("./build/test.ll").expect("faild");
  }
}
