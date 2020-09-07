; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %putchar = call i32 @putchar(i32 104)
  %putchar1 = call i32 @putchar(i32 101)
  %putchar2 = call i32 @putchar(i32 108)
  %putchar3 = call i32 @putchar(i32 108)
  %putchar4 = call i32 @putchar(i32 111)
  %putchar5 = call i32 @putchar(i32 10)
  ret i32 0
}
