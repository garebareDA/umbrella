; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %return = call i32 @a()
  ret i32 0
}

define i32 @a() {
entry:
  %putchar = call i32 @putchar(i32 104)
  %putchar1 = call i32 @putchar(i32 101)
  %putchar2 = call i32 @putchar(i32 108)
  %putchar3 = call i32 @putchar(i32 108)
  %putchar4 = call i32 @putchar(i32 111)
  %putchar5 = call i32 @putchar(i32 32)
  %putchar6 = call i32 @putchar(i32 119)
  %putchar7 = call i32 @putchar(i32 111)
  %putchar8 = call i32 @putchar(i32 114)
  %putchar9 = call i32 @putchar(i32 108)
  %putchar10 = call i32 @putchar(i32 100)
  %putchar11 = call i32 @putchar(i32 10)
  ret i32 0
}
