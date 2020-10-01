use super::super::parser::ast;
use super::compile::CodeGen;
use inkwell::basic_block;

impl<'ctx> CodeGen<'ctx> {
  pub fn for_write(
    &mut self,
    fors: &ast::ForsAST,
    basic_block: basic_block::BasicBlock,
  ) -> Result<(), String> {
    self.push_var_vec();
    self.push_fun_vec();

    let i32_type = self.context.i32_type();
    let (types, name_vec) = self.vars_type();
    let main_type = i32_type.fn_type(&types, false);
    let function = self.module.add_function("fors", main_type, None);
    let function_param = function.get_params();
    match self.push_var_param(function_param, &name_vec) {
      Ok(()) => {}
      Err(s) => {
        return Err(s);
      }
    }
    let basic_block_entry = self.context.append_basic_block(function, "entry");
    let basic_block_preloop = self.context.append_basic_block(function, "preloop");
    let basic_block_loop = self.context.append_basic_block(function, "loop");
    let basic_block_afterloop = self.context.append_basic_block(function, "afterloop");

    self.builder.position_at_end(basic_block_entry);
    self.builder.build_unconditional_branch(basic_block_preloop);

    self.builder.position_at_end(basic_block_preloop);
    let init = self.fors_init_inner(&fors.init[0]);
    match init {
      Ok((var_name, num_i32)) => {
        let variable = self.builder.build_phi(i32_type, &var_name);
        variable.add_incoming(&[(&num_i32, basic_block_entry)]);
        self.push_var(variable.as_basic_value(), &var_name);
        let for_ifs = self.fors_ifs_init(&fors.ifs[0]);

        let sum = self.calcuration(&for_ifs.unwrap());
        match sum {
          Ok(sum) => {
            self
              .builder
              .build_conditional_branch(sum, basic_block_loop, basic_block_afterloop);
          }

          Err(s) => {
            return Err(s);
          }
        }

        self.builder.position_at_end(basic_block_loop);
        let scope = self.scope_write(&fors.node, basic_block_loop);
        match scope {
          Ok(_) => {}
          Err(s) => {
            return Err(s);
          }
        }
        let for_count = self.fors_ifs_init(&fors.count[0]);

        let sum = self.calcuration(&for_count.unwrap());
        match sum {
          Ok(sum) => {
            variable.add_incoming(&[(&sum, basic_block_loop)]);
          }

          Err(s) => {
            return Err(s);
          }
        }

        self.builder.build_unconditional_branch(basic_block_preloop);
        self.builder.position_at_end(basic_block_afterloop);
        self
          .builder
          .build_return(Some(&i32_type.const_int(0, false)));

        self.push_var_vec_remove();
        self.push_fun_vec_remove();

        self.builder.position_at_end(basic_block);
        self
          .builder
          .build_call(function, &self.get_argment(&name_vec).unwrap(), "fors");
      }

      Err(s) => {
        return Err(s);
      }
    }
    return Ok(());
  }

  fn fors_init_inner(
    &self,
    init: &ast::Types,
  ) -> Result<(String, inkwell::values::IntValue), String> {
    let i32_type = self.context.i32_type();
    match init {
      ast::Types::Variable(vars) => match &vars.node[0] {
        ast::Types::Number(num) => {
          let num_i32 = i32_type.const_int(num.num as u64, false);
          return Ok((vars.name.to_string(), num_i32));
        }

        ast::Types::Binary(bin) => {
          let num_i32 = self.calcuration(bin);
          match num_i32 {
            Ok(num_i32) => {
              return Ok((vars.name.to_string(), num_i32));
            }

            Err(s) => {
              return Err(s);
            }
          }
        }
        _ => Err(format!("{} not a number", vars.name)),
      },
      _ => Err("for init error".to_string()),
    }
  }

  fn fors_ifs_init(&self, ifs: &ast::Types) -> Result<ast::BinaryAST, String> {
    match ifs {
      ast::Types::Binary(bin) => {
        return Ok(bin.clone());
      }
      _ => Err("for binary error".to_string()),
    }
  }
}
