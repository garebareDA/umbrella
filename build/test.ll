; ModuleID = 'main'
source_filename = "main"

@format = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@format.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@strings = private unnamed_addr constant [7 x i8] c"hello\0A\00"

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %return = call i32 @b(i32 1)
  %sum = add i32 %return, 1
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format.1, i32 0, i32 0), i32 %sum)
  %fors = call i32 @fors()
  ret i32 0
}

define i32 @b(i32) {
entry:
  %sum = add i32 %0, 1
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format, i32 0, i32 0), i32 %sum)
  ret i32 %0
  ret i32 0
}

define i32 @fors() {
entry:
  br label %preloop

preloop:                                          ; preds = %loop, %entry
  %i = phi i32 [ 1, %entry ], [ %sum, %loop ]
  %lessthan = icmp slt i32 %i, 5
  br i1 %lessthan, label %loop, label %afterloop

loop:                                             ; preds = %preloop
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @strings, i32 0, i32 0))
  %sum = add i32 %i, 1
  br label %preloop

afterloop:                                        ; preds = %preloop
  ret i32 0
}
