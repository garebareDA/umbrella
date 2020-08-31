use super::super::parser::ast;
use super::compile::CodeGen;

impl<'ctx> CodeGen<'ctx> {
  pub fn for_write(&mut self, fors: &ast::ForsAST) {
    let i32_type = self.context.i32_type();
    let main_type = i32_type.fn_type(&[], false);
    let function = self.module.add_function("fors", main_type, None);
    let basic_block_entry = self.context.append_basic_block(function, "entry");
    let basic_block_preloop = self.context.append_basic_block(function, "preloop");
    let basic_block_loop = self.context.append_basic_block(function, "loop");
    let basic_block_afterloop = self.context.append_basic_block(function, "afterloop");

    self.builder.position_at_end(basic_block_entry);
    self.builder.build_unconditional_branch(basic_block_preloop);
    self.builder.position_at_end(basic_block_preloop);

    let (var_name, num_i32) = self.fors_init_inner(&fors.init[0]).unwrap();
    let variable = self.builder.build_phi(i32_type, &var_name);
    variable.add_incoming(&[(&num_i32, basic_block_entry)]);
    

    self.builder.position_at_end(basic_block_loop);
    for ast in fors.node.iter() {
      self.judge(&ast);
    }
    self.builder.build_unconditional_branch(basic_block_preloop);

    self.builder.position_at_end(basic_block_afterloop);
    self
      .builder
      .build_return(Some(&i32_type.const_int(0, false)));
  }

  fn fors_init_inner(&self, init:&ast::Types) -> Result<(String, inkwell::values::IntValue), ()> {
    let i32_type = self.context.i32_type();
    match init {
      ast::Types::Variable(vars) => {
        match &vars.node[0]{
          ast::Types::Number(num) => {
            let num_i32 = i32_type.const_int(num.num as u64, false);
            return Ok((vars.name.to_string(), num_i32));
          }

          ast::Types::Binary(bin) => {
            let num_i32 = self.calcuration(bin);
            return Ok((vars.name.to_string(), num_i32));
          }
          _ => {Err(())}
        }
      }
      _ => {Err(())}
    }
  }
}
