use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::basic_block;

impl<'ctx> CodeGen<'ctx> {
  pub fn if_write(
    &mut self,
    ifs: &ast::IfsAST,
    basic_block: basic_block::BasicBlock,
  ) -> Result<(), String> {
    self.push_var_vec();
    self.push_fun_vec();

    let i32_type = self.context.i32_type();
    let (types, name_vec) = self.vars_type();
    let main_type = i32_type.fn_type(&types, false);
    let function = self.module.add_function("ifs", main_type, None);
    let function_param = function.get_params();
    self.push_var_param(function_param, &name_vec);
    let basic_block_entry = self.context.append_basic_block(function, "entry");

    match &ifs.ifs[0] {
      ast::Types::Binary(bin) => {
        let basic_block_then = self.context.append_basic_block(function, "then");
        self.builder.position_at_end(basic_block_then);
        let scope = self.scope_write(&ifs.then, basic_block_then);
        match scope {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }

        let basic_block_else = self.context.append_basic_block(function, "else");
        self.builder.position_at_end(basic_block_else);
        let scope = self.scope_write(&ifs.elses, basic_block_else);
        match scope {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }

        self.builder.position_at_end(basic_block_entry);
        let sum = self.calcuration(&bin);
        match sum {
          Ok(sum) => {
            self
              .builder
              .build_conditional_branch(sum, basic_block_then, basic_block_else);
          }

          Err(s) => {
            return Err(s);
          }
        }

        let basic_block_end = self.context.append_basic_block(function, "end");
        self.builder.position_at_end(basic_block_end);
        self
          .builder
          .build_return(Some(&i32_type.const_int(0, false)));

        self.builder.position_at_end(basic_block_then);
        self.builder.build_unconditional_branch(basic_block_end);
        self.builder.position_at_end(basic_block_else);
        self.builder.build_unconditional_branch(basic_block_end);

        self.push_var_vec_remove();
        self.push_fun_vec_remove();

        self.builder.position_at_end(basic_block);
        self
          .builder
          .build_call(function, &self.get_argment(&name_vec).unwrap(), "ifs");
      }

      _ => return Err("not found oprater".to_string()),
    }

    return Ok(());
  }
}
