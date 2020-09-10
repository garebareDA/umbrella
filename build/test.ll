; ModuleID = 'main'
source_filename = "main"

@format = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@strings = private unnamed_addr constant [6 x i8] c"test\0A\00"
@format.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@strings.2 = private unnamed_addr constant [7 x i8] c"hello\0A\00"

declare i32 @putchar(i32)

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @strings, i32 0, i32 0))
  %printf1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format.1, i32 0, i32 0), i32 2)
  %return = call i32 @b(i32 1)
  %printf2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @strings.2, i32 0, i32 0))
  ret i32 0
}

define i32 @b(i32) {
entry:
  %sum = add i32 %0, 1
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format, i32 0, i32 0), i32 %sum)
  ret i32 0
}
