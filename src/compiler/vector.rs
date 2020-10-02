use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::types::VectorType;
use inkwell::values;

impl<'ctx> CodeGen<'ctx> {
  pub fn vector_write(&self, vec: &ast::VectorAST, types: &ast::VariableType) -> Result<values::BasicValueEnum<'ctx>, String>{
    match types {
      ast::VariableType::Int => {
        let i32_type = self.context.i32_type();
        let mut vec_inner:Vec<inkwell::values::IntValue> = Vec::new();
        for inner in vec.vec.iter() {
          match inner {
            ast::Types::Number(num) => {
              let int = i32_type.const_int(num.num as u64, false);
              vec_inner.push(int);
            }
            _ => {}
          }
        }
        let i32_vec = VectorType::const_vector(&vec_inner);
        return Ok(values::BasicValueEnum::VectorValue(i32_vec));
      },

      ast::VariableType::Strings => {}

      _ => {}
    }

    return Err(format!("vector error"));
  }
}
