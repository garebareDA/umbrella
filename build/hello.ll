; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %putchar = call i32 @putchar(i32 105)
  %putchar1 = call i32 @putchar(i32 51)
  %putchar2 = call i32 @putchar(i32 50)
  %putchar3 = call i32 @putchar(i32 32)
  %putchar4 = call i32 @putchar(i32 49)
  %putchar5 = call i32 @putchar(i32 52)
  %putchar6 = call i32 @putchar(i32 10)
  ret i32 0
}

declare i32 @putchar(i32)
