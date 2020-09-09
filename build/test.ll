; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %putchar = call i32 @putchar(i32 105)
  %putchar1 = call i32 @putchar(i32 51)
  %putchar2 = call i32 @putchar(i32 50)
  %putchar3 = call i32 @putchar(i32 32)
  %putchar4 = call i32 @putchar(i32 50)
  %putchar5 = call i32 @putchar(i32 10)
  ret i32 0
}
