; ModuleID = 'main'
source_filename = "main"

@format = private unnamed_addr constant [4 x i8] c"%d\0A\00"

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  ret i32 0
}

define i32 @b(i32) {
entry:
  %sum = add i32 %0, 1
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format, i32 0, i32 0), i32 %sum)
  ret i32 %0
  ret i32 0
}
