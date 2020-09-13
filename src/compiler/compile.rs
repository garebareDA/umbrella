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
  code_gen.add_fun_printf();
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
              self.print(values::BasicValueEnum::IntValue(sum));
            }

            ast::Types::Variable(var) => match self.vars_serch(&var.name) {
              Ok(var) => {
                self.print(*var);
              }
              Err(()) => {}
            },
            _ => {}
          }
        }
        self.call_write(call);
      }

      ast::Types::Ifs(ifs) => {
        self.if_write(&ifs, basic_block);
      }

      ast::Types::Fors(fors) => {
        self.for_write(&fors, basic_block);
      }

      ast::Types::Variable(var) => {
        if !var.node.is_empty() {
          self.var_write(&var.name, &var.node[0]);
        }
      }

      ast::Types::Return(ret) => {
        self.return_write(&ret.node[0]);
      }

      _ => {}
    }
  }

  pub fn scope_write(
    &mut self,
    node: &Vec<ast::Types>,
    basic_block: inkwell::basic_block::BasicBlock,
  ) {
    for ast in node.iter() {
      self.builder.position_at_end(basic_block);
      self.judge(ast, basic_block);
    }
  }

  fn start_function_write(
    &mut self,
    node: &Vec<ast::Types>,
    basic_block: inkwell::basic_block::BasicBlock,
  ) {
    for ast in node.iter() {
      self.builder.position_at_end(basic_block);
      match ast {
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
  }

  fn set_main_run(&mut self, node: &Vec<ast::Types>) {
    self.push_var_vec();
    self.push_fun_vec();
    let i32_type = self.context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let function = self.module.add_function("main", main_type, None);
    let basic_block = self.context.append_basic_block(function, "entry");
    self.start_function_write(node, basic_block);
    self.scope_write(node, basic_block);
    self.builder.position_at_end(basic_block);
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
