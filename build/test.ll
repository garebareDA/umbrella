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
  %return = call i32 @b(i32 1)
  ret i32 0
}

define i32 @b(i32) {
entry:
  %sum = add i32 %0, 1
  %putchar = call i32 @putchar(i32 32)
  %putchar1 = call i32 @putchar(i32 32)
  %putchar2 = call i32 @putchar(i32 37)
  %putchar3 = call i32 @putchar(i32 115)
  %putchar4 = call i32 @putchar(i32 117)
  %putchar5 = call i32 @putchar(i32 109)
  %putchar6 = call i32 @putchar(i32 32)
  %putchar7 = call i32 @putchar(i32 61)
  %putchar8 = call i32 @putchar(i32 32)
  %putchar9 = call i32 @putchar(i32 97)
  %putchar10 = call i32 @putchar(i32 100)
  %putchar11 = call i32 @putchar(i32 100)
  %putchar12 = call i32 @putchar(i32 32)
  %putchar13 = call i32 @putchar(i32 105)
  %putchar14 = call i32 @putchar(i32 51)
  %putchar15 = call i32 @putchar(i32 50)
  %putchar16 = call i32 @putchar(i32 32)
  %putchar17 = call i32 @putchar(i32 37)
  %putchar18 = call i32 @putchar(i32 48)
  %putchar19 = call i32 @putchar(i32 44)
  %putchar20 = call i32 @putchar(i32 32)
  %putchar21 = call i32 @putchar(i32 49)
  %putchar22 = call i32 @putchar(i32 10)
  ret i32 0
}
