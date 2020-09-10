extern crate inkwell;

use inkwell::values;

use super::super::parser::ast;
use super::super::parser::ast::Types;
use super::compile::CodeGen;

impl<'ctx> CodeGen<'ctx> {
  pub fn calcuration(&self, bin: &ast::BinaryAST) -> values::IntValue<'ctx> {
    let mut op_stack: Vec<char> = Vec::new();
    let mut number_stack: Vec<values::BasicValueEnum> = Vec::new();
    let op = bin.op;
    op_stack.push(op);

    match &bin.node[0] {
      Types::Number(num) => {
        let num_i32 = self.context.i32_type();
        let int = num_i32.const_int(num.num as u64, false);
        number_stack.push(values::BasicValueEnum::IntValue(int));
      }

      Types::Variable(vars) => {
        match self.vars_serch(&vars.name) {
          Ok(var) => {
            number_stack.push(var.clone());
          }
          Err(()) => {}
        }
      }

      _ => {
        //error
      }
    }

    match &bin.node[1] {
      Types::Number(num) => {
        let num_i32 = self.context.i32_type();
        let int = num_i32.const_int(num.num as u64, false);
        number_stack.push(values::BasicValueEnum::IntValue(int));
        if bin.node.len() > 1 && !num.node.is_empty() {
          self.calcuration_stack(&mut op_stack, &mut number_stack, &num.node[0]);
        }
      }

      Types::Binary(bin) => {
        op_stack.push(bin.op);
      }

      _ => {
        //error
      }
    }

    if op_stack.len() == 2 && number_stack.len() == 1 {
      let num_i32 = self.context.i32_type();
      if op_stack[0] == '+' && op_stack[1] == '+' {
        let l_stack = number_stack[0].into_int_value();
        let r_stack = num_i32.const_int(1 as u64, false);
        let sum = self.builder.build_int_add(l_stack, r_stack, "sum");
        return sum;
      }

      if op_stack[0] == '-' && op_stack[1] == '-' {
        let l_stack = number_stack[0].into_int_value();
        let r_stack = num_i32.const_int(1 as u64, false);
        let sum = self.builder.build_int_sub(l_stack, r_stack, "sub");
        return sum;
      }
    }

    let mut cal_counter = 0;
    for (i, op) in op_stack.iter().enumerate() {
      if number_stack.len() == 1 {
        break;
      }
      let l_index = i - cal_counter;
      let r_index = i - cal_counter + 1;

      let l_stack = number_stack[l_index].into_int_value();
      let r_stack = number_stack[r_index].into_int_value();

      if op == &'/' {
        let sum = self.builder.build_int_unsigned_div(l_stack, r_stack, "div");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == &'*' {
        let sum = self.builder.build_int_mul(l_stack, r_stack, "mul");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      cal_counter += 1;
    }

    let mut cal_counter = 0;
    for (i, op) in op_stack.iter().enumerate() {
      if number_stack.len() == 1 {
        break;
      }
      let l_index = i - cal_counter;
      let r_index = i - cal_counter + 1;

      let l_stack = number_stack[l_index].into_int_value();
      let r_stack = number_stack[r_index].into_int_value();

      if op == &'+' {
        let sum = self.builder.build_int_add(l_stack, r_stack, "sum");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == &'-' {
        let sum = self.builder.build_int_sub(l_stack, r_stack, "sub");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      cal_counter += 1;
    }

    let mut cal_counter = 0;
    for (i, op) in op_stack.iter().enumerate() {
      if number_stack.len() == 1 {
        break;
      }
      let l_index = i - cal_counter;
      let r_index = i - cal_counter + 1;

      let l_stack = number_stack[l_index].into_int_value();
      let r_stack = number_stack[r_index].into_int_value();

      if op == &'<' {
        let sum =
          self
            .builder
            .build_int_compare(inkwell::IntPredicate::SLT, l_stack, r_stack, "lessthan");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == &'>' {
        let sum = self.builder.build_int_compare(
          inkwell::IntPredicate::SGT,
          l_stack,
          r_stack,
          "greaterthan",
        );
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      cal_counter += 1;
    }

    return number_stack[0].into_int_value();
  }

  fn calcuration_stack(
    &self,
    op_stack: &mut Vec<char>,
    number_stack: &mut Vec<values::BasicValueEnum<'ctx>>,
    types: &ast::Types,
  ) {
    let num_i32 = self.context.i32_type();
    match types {
      ast::Types::Number(num) => {
        let int = num_i32.const_int(num.num as u64, false);
        number_stack.push(values::BasicValueEnum::IntValue(int));
        if !&num.node.is_empty() {
          self.calcuration_stack(op_stack, number_stack, &num.node[0]);
        }
      }

      ast::Types::Binary(op) => {
        op_stack.push(op.op);
        if !&op.node.is_empty() {
          self.calcuration_stack(op_stack, number_stack, &op.node[0]);
        }
      }

      ast::Types::Variable(vars) => match self.vars_serch(&vars.name) {
        Ok(var) => {
          number_stack.push(var.clone());
          if !&vars.node.is_empty() {
            self.calcuration_stack(op_stack, number_stack, &vars.node[0]);
          }
        }

        Err(()) => {}
      },
      _ => {}
    }
  }
}
