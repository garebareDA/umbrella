use super::compile::CodeGen;
use inkwell::values;

impl<'ctx> CodeGen<'ctx> {
  pub fn print(&self, print: values::BasicValueEnum) {
    match print {
      values::BasicValueEnum::IntValue(_) => {
        let format = self.builder.build_global_string_ptr("%d\n", "format");
        let printf = self.module.get_function("printf");
        self.builder.build_call(
          printf.unwrap(),
          &[
            values::BasicValueEnum::PointerValue(format.as_pointer_value()),
            print,
          ],
          "printf",
        );
      }

      values::BasicValueEnum::PointerValue(_) => {
        let printf = self.module.get_function("printf");
        self.builder.build_call(
          printf.unwrap(),
          &[
            print,
          ],
          "printf",
        );
      }
      _ => {}
    }
  }
}
