use super::compile::CodeGen;
use super::super::parser::ast;

impl<'ctx> CodeGen<'ctx> {
  pub fn call_write(&self, call: &ast::CallAST) {
    let function = self.module.get_function(&call.callee);
    
  }
}