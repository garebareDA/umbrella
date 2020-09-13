extern crate inkwell;

use inkwell::values;

use super::super::parser::ast;
use super::super::parser::ast::Types;
use super::compile::CodeGen;

impl<'ctx> CodeGen<'ctx> {
  pub fn calcuration(&self, bin: &ast::BinaryAST) -> Result<values::IntValue<'ctx>, String> {
    let mut op_stack: Vec<String> = Vec::new();
    let mut number_stack: Vec<values::BasicValueEnum> = Vec::new();
    let op = &bin.op;
    op_stack.push(op.to_string());

    match &bin.node[0] {
      Types::Number(num) => {
        let num_i32 = self.context.i32_type();
        let int = num_i32.const_int(num.num as u64, false);
        number_stack.push(values::BasicValueEnum::IntValue(int));
      }

      Types::Variable(vars) => match self.vars_serch(&vars.name) {
        Ok(var) => {
          number_stack.push(var.clone());
        }
        Err(s) => {
          return Err(s);
        }
      },

      _ => {
        return Err("calculation formula type error".to_string());
      }
    }

    match &bin.node[1] {
      Types::Number(num) => {
        let num_i32 = self.context.i32_type();
        let int = num_i32.const_int(num.num as u64, false);
        number_stack.push(values::BasicValueEnum::IntValue(int));
        if bin.node.len() > 1 && !num.node.is_empty() {
          let result = self.calcuration_stack(&mut op_stack, &mut number_stack, &num.node[0]);
          match result {
            Ok(_) => {}
            Err(s) => {
              return Err(s);
            }
          }
        }
      }

      Types::Binary(bin) => {
        op_stack.push(bin.op.to_string());
      }

      _ => {
        return Err("calculation formula type error".to_string());
      }
    }

    if op_stack.len() == 2 && number_stack.len() == 1 {
      let num_i32 = self.context.i32_type();
      if op_stack[0] == "+" && op_stack[1] == "+" {
        let l_stack = number_stack[0].into_int_value();
        let r_stack = num_i32.const_int(1 as u64, false);
        let sum = self.builder.build_int_add(l_stack, r_stack, "sum");
        return Ok(sum);
      }

      if op_stack[0] == "-" && op_stack[1] == "-" {
        let l_stack = number_stack[0].into_int_value();
        let r_stack = num_i32.const_int(1 as u64, false);
        let sum = self.builder.build_int_sub(l_stack, r_stack, "sub");
        return Ok(sum);
      }
    }
    self.calcuration_op(&op_stack, &mut number_stack, 1);
    self.calcuration_op(&op_stack, &mut number_stack, 2);
    self.calcuration_op(&op_stack, &mut number_stack, 3);

    return Ok(number_stack[0].into_int_value());
  }

  fn calcuration_stack(
    &self,
    op_stack: &mut Vec<String>,
    number_stack: &mut Vec<values::BasicValueEnum<'ctx>>,
    types: &ast::Types,
  ) -> Result<(), String> {
    let num_i32 = self.context.i32_type();
    match types {
      ast::Types::Number(num) => {
        let int = num_i32.const_int(num.num as u64, false);
        number_stack.push(values::BasicValueEnum::IntValue(int));
        if !&num.node.is_empty() {
          let result = self.calcuration_stack(op_stack, number_stack, &num.node[0]);
          match result {
            Ok(_) => {}
            Err(s) => {
              return Err(s);
            }
          }
        }
      }

      ast::Types::Binary(op) => {
        op_stack.push(op.op.to_string());
        if !&op.node.is_empty() {
          let result = self.calcuration_stack(op_stack, number_stack, &op.node[0]);
          match result {
            Ok(_) => {}
            Err(s) => {
              return Err(s);
            }
          }
        }
      }

      ast::Types::Variable(vars) => match self.vars_serch(&vars.name) {
        Ok(var) => {
          number_stack.push(var.clone());
          if !&vars.node.is_empty() {
            let result = self.calcuration_stack(op_stack, number_stack, &vars.node[0]);
            match result {
              Ok(_) => {}
              Err(s) => {
                return Err(s);
              }
            }
          }
        }

        Err(s) => {
          return Err(s);
        }
      },
      _ => return Err("calculation stack error".to_string()),
    }
    return Ok(());
  }

  fn calcuration_op(
    &self,
    op_stack: &Vec<String>,
    number_stack: &mut Vec<values::BasicValueEnum<'ctx>>,
    ops: usize,
  ) {
    let before_len = number_stack.len();
    for (i, op) in op_stack.iter().enumerate() {
      let sub_len = before_len - number_stack.len();
      let l_index = i - sub_len;
      let r_index = i + 1 - sub_len;

      if number_stack.len() == 1 || number_stack.len() <= r_index {
        break;
      }

      let l_stack = number_stack[l_index].into_int_value();
      let r_stack = number_stack[r_index].into_int_value();

      if op == "*" && 1 == ops {
        let sum = self.builder.build_int_mul(l_stack, r_stack, "mul");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == "/" && 1 == ops {
        let sum = self.builder.build_int_unsigned_div(l_stack, r_stack, "div");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == "+" && 2 == ops {
        let sum = self.builder.build_int_add(l_stack, r_stack, "sum");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == "-" && 2 == ops {
        let sum = self.builder.build_int_sub(l_stack, r_stack, "sub");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == "<" && 3 == ops {
        let sum =
          self
            .builder
            .build_int_compare(inkwell::IntPredicate::SLT, l_stack, r_stack, "lessthan");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == ">" && 3 == ops {
        let sum = self.builder.build_int_compare(
          inkwell::IntPredicate::SGT,
          l_stack,
          r_stack,
          "greaterthan",
        );
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == "<=" && 3 == ops {
        let sum = self.builder.build_int_compare(
          inkwell::IntPredicate::SLE,
          l_stack,
          r_stack,
          "greaterthanorequal",
        );
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == ">=" && 3 == ops {
        let sum = self.builder.build_int_compare(
          inkwell::IntPredicate::SGE,
          l_stack,
          r_stack,
          "lessthanorequal",
        );
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == "==" && 3 == ops {
        let sum =
          self
            .builder
            .build_int_compare(inkwell::IntPredicate::EQ, l_stack, r_stack, "equal");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }

      if op == "!=" && 3 == ops {
        let sum =
          self
            .builder
            .build_int_compare(inkwell::IntPredicate::NE, l_stack, r_stack, "notequal");
        number_stack[l_index] = values::BasicValueEnum::IntValue(sum);
        number_stack.remove(r_index);
      }
    }
  }
}
