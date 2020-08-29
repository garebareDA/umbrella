; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

define i32 @fors() {
entry:
  br label %preloop

preloop:                                          ; preds = %entry
  %i = phi i32 [ 1, %entry ]

loop:                                             ; No predecessors!

afterloop:                                        ; No predecessors!
}

define i32 @main() {
entry:
  ret i32 0
}
