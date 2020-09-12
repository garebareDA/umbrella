use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::basic_block;

impl<'ctx> CodeGen<'ctx> {
  pub fn if_write(&mut self, ifs: &ast::IfsAST, basic_block: basic_block::BasicBlock) {
    let i32_type = self.context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let function = self.module.add_function("ifs", main_type, None);
    let basic_block_entry = self.context.append_basic_block(function, "entry");

    match &ifs.ifs[0] {
      ast::Types::Binary(bin) => {
        let basic_block_then = self.context.append_basic_block(function, "then");
        self.builder.position_at_end(basic_block_then);
        self.scope_write(&ifs.then, basic_block_then);

        let basic_block_else = self.context.append_basic_block(function, "else");
        self.builder.position_at_end(basic_block_else);
        self.scope_write(&ifs.elses, basic_block_else);

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

        self.builder.position_at_end(basic_block);
        self.builder.build_call(function, &[], "ifs");
      }

      _ => {}
    }
  }
}
