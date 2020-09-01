; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @main() {
entry:
  %ifs = call i32 @ifs()
  ret i32 0
}

define i32 @fors() {
entry:
  br label %preloop

preloop:                                          ; preds = %loop, %entry
  %i = phi i32 [ 0, %entry ]

loop:                                             ; No predecessors!
  %putchar = call i32 @putchar(i32 104)
  %putchar1 = call i32 @putchar(i32 101)
  %putchar2 = call i32 @putchar(i32 108)
  %putchar3 = call i32 @putchar(i32 108)
  %putchar4 = call i32 @putchar(i32 111)
  %putchar5 = call i32 @putchar(i32 10)
  br label %preloop

afterloop:                                        ; No predecessors!
  ret i32 0
}

define i32 @ifs() {
entry:
  br i1 true, label %then, label %else

then:                                             ; preds = %entry
  %putchar = call i32 @putchar(i32 104)
  %putchar1 = call i32 @putchar(i32 101)
  %putchar2 = call i32 @putchar(i32 108)
  %putchar3 = call i32 @putchar(i32 108)
  %putchar4 = call i32 @putchar(i32 111)
  %putchar5 = call i32 @putchar(i32 10)
  br label %end

else:                                             ; preds = %entry
  br label %end

end:                                              ; preds = %else, %then
  ret i32 0
}
