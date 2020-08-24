extern crate inkwell;

use inkwell::values;

use super::super::parser::ast;
use super::super::parser::ast::Types;
use super::compile::CodeGen;

impl<'ctx> CodeGen<'ctx> {
  pub fn calcuration(&self, bin: &ast::BinaryAST) -> values::IntValue {
    let num_i32 = self.context.i64_type();
    let mut op_stack: Vec<char> = Vec::new();
    let mut number_stack: Vec<values::IntValue> = Vec::new();
    let op = bin.op;
    op_stack.push(op);

    match &bin.node[0] {
      Types::Number(num) => {
        number_stack.push(num_i32.const_int(num.num as u64, false));
      }

      _ => {
        //error
      }
    }

    match &bin.node[1] {
      Types::Number(num) => {
        number_stack.push(num_i32.const_int(num.num as u64, false));
        self.calcuration_stack(&mut op_stack, &mut number_stack, &num.node[0]);
      }

      _ => {
        //error
      }
    }

    let mut cal_counter = 0;
    for (i, op) in op_stack.iter().enumerate() {
      let l_index = i - cal_counter;
      let r_index = i - cal_counter + 1;

      let l_stack = number_stack[l_index];
      let r_stack=number_stack[r_index];

      if op == &'/'{
        let sum = self.builder.build_int_unsigned_div(l_stack, r_stack, "div");
        number_stack[l_index] = sum;
        number_stack.remove(r_index);
      }

      if op == &'*'{
        let sum = self.builder.build_int_mul(l_stack, r_stack, "mul");
        number_stack[l_index] = sum;
        number_stack.remove(r_index);
      }

      cal_counter += 1;
    }

    let mut cal_counter = 0;
    for (i, op) in op_stack.iter().enumerate() {
      let l_index = i - cal_counter;
      let r_index = i - cal_counter + 1;

      let l_stack = number_stack[l_index];
      let r_stack =number_stack[r_index];

      if op == &'+'{
       let sum = self.builder.build_int_add(l_stack, r_stack, "add");
       number_stack[l_index] = sum;
       number_stack.remove(r_index);
     }

     if op == &'-'{
       let sum = self.builder.build_int_sub(l_stack, r_stack, "sub");
       number_stack[l_index] = sum;
       number_stack.remove(r_index);
     }

     cal_counter += 1;
   }

   return number_stack[0];
  }

  fn calcuration_stack (&self, op_stack: &mut Vec<char>, number_stack: &mut Vec<values::IntValue<'ctx>>, types: &ast::Types) {
    match types {
      ast::Types::Number(num) => {
        let num_i32 = self.context.i32_type();
        number_stack.push(num_i32.const_int(num.num as u64, false));
        if !&num.node.is_empty(){
          self.calcuration_stack(op_stack, number_stack, &num.node[0]);
        }
      }

      ast::Types::Binary(op) => {
        op_stack.push(op.op);
        if !&op.node.is_empty(){
          self.calcuration_stack(op_stack, number_stack, &op.node[0]);
        }
      }

      _ => {}
    }
  }
}
