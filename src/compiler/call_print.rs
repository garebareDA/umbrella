use super::compile::CodeGen;
use inkwell::values;

impl<'ctx> CodeGen<'ctx> {
  pub fn print_string(&self, strings:&str){
    let i32_type = self.context.i32_type();
    let putchar = self.module.get_function("putchar");
    let word = strings.to_string() + "\n";
    for c in word.chars() {
      let ascii = c.to_string().as_bytes()[0] as u64;
      self.builder.build_call(
        putchar.unwrap(),
        &[i32_type.const_int(ascii, false).into()],
        "putchar",
      );
    }
  }

  pub fn print_number(&self, num:values::BasicValueEnum) {
    let format = self.builder.build_global_string_ptr("%f\n", "format");
    let printf = self.module.get_function("printf");
    self.builder.build_call(printf.unwrap(),&[values::BasicValueEnum::PointerValue(format.as_pointer_value()), num], "printf");
  }
}