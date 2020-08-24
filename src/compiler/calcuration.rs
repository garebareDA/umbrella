extern crate inkwell;

use inkwell::values;

use super::super::parser::ast;
use super::super::parser::ast::Types;
use super::compile::CodeGen;

impl<'ctx> CodeGen<'ctx> {
  pub fn calcuration(&self, bin: &ast::BinaryAST) -> values::IntValue {
    let num_i32 = self.context.i32_type();
    let mut op_stack: Vec<char> = Vec::new();
    let mut number_stack: Vec<values::IntValue> = Vec::new();
    let op = bin.op;
    op_stack.push(op);

    match &bin.node[1] {
      Types::Number(num) => {
        number_stack.push(num_i32.const_int(num.num as u64, false));
      }

      _ => {
        //error
      }
    }

    match &bin.node[0] {
      Types::Number(num) => {
        number_stack.push(num_i32.const_int(num.num as u64, false));
        self.calcuration_stack(&mut op_stack, &mut number_stack, &num.node[0]);
      }

      _ => {
        //error
      }
    }

    for (i, op) in op_stack.iter().enumerate() {
      if op == &'/'{
        let sum = self.builder.build_int_unsigned_div(number_stack[i], number_stack[i + 1], "div");
        number_stack[i] = sum;
        number_stack.remove(i + 1);
      }

      if op == &'*'{
        let sum = self.builder.build_int_mul(number_stack[i], number_stack[i + 1], "mul");
        number_stack[i] = sum;
        number_stack.remove(i + 1);
      }
    }

    for (i, op) in op_stack.iter().enumerate() {
      if op == &'+'{
       let sum = self.builder.build_int_add(number_stack[i], number_stack[i + 1], "add");
       number_stack[i] = sum;
       number_stack.remove(i + 1);
     }

     if op == &'-'{
       let sum = self.builder.build_int_sub(number_stack[i], number_stack[i + 1], "sub");
       number_stack[i] = sum;
       number_stack.remove(i + 1);
     }
   }

   return number_stack[0];
  }

  fn calcuration_stack (&self, op_stack: &mut Vec<char>, number_stack: &mut Vec<values::IntValue<'ctx>>, types: &ast::Types) {
    match types {
      ast::Types::Number(num) => {
        let num_i32 = self.context.i32_type();
        number_stack.push(num_i32.const_int(num.num as u64, false));
        self.calcuration_stack(op_stack, number_stack, &num.node[0]);
      }

      ast::Types::Binary(op) => {
        op_stack.push(op.op);
        self.calcuration_stack(op_stack, number_stack, &op.node[0]);
      }

      _ => {}
    }
  }
}
