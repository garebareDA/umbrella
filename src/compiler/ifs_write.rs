use super::super::parser::ast;
use super::compile::CodeGen;

impl<'ctx> CodeGen<'ctx> {
  pub fn if_write(&mut self, ifs: &ast::IfsAST) {
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
}
