; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %fors = call i32 @fors()
  ret i32 0
}

define i32 @fors() {
entry:
  br label %preloop

preloop:                                          ; preds = %loop, %entry
  %i = phi i32 [ 0, %entry ], [ %sum, %loop ]
  %lessthan = icmp slt i32 %i, 5
  br i1 %lessthan, label %loop, label %afterloop

loop:                                             ; preds = %preloop
  %putchar = call i32 @putchar(i32 104)
  %putchar1 = call i32 @putchar(i32 101)
  %putchar2 = call i32 @putchar(i32 108)
  %putchar3 = call i32 @putchar(i32 108)
  %putchar4 = call i32 @putchar(i32 111)
  %putchar5 = call i32 @putchar(i32 10)
  %sum = add i32 %i, 1
  br label %preloop

afterloop:                                        ; preds = %preloop
  ret i32 0
}
