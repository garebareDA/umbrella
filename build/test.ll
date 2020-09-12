; ModuleID = 'main'
source_filename = "main"

@strings = private unnamed_addr constant [10 x i8] c"function\0A\00"
@format = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@format.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@strings.2 = private unnamed_addr constant [6 x i8] c"else\0A\00"
@format.3 = private unnamed_addr constant [4 x i8] c"%d\0A\00"
@format.4 = private unnamed_addr constant [4 x i8] c"%d\0A\00"

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format, i32 0, i32 0), i32 2)
  %ifs = call i32 @ifs()
  %fors = call i32 @fors()
  %return = call i32 @function()
  %printf1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format.4, i32 0, i32 0), i32 %return)
  ret i32 0
}

define i32 @function() {
entry:
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @strings, i32 0, i32 0))
  ret i32 1
  ret i32 0
}

define i32 @ifs() {
entry:
  br i1 false, label %then, label %else

then:                                             ; preds = %entry
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format.1, i32 0, i32 0), i32 2)
  br label %end

else:                                             ; preds = %entry
  %printf1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @strings.2, i32 0, i32 0))
  br label %end

end:                                              ; preds = %else, %then
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
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @format.3, i32 0, i32 0), i32 %i)
  %sum = add i32 %i, 1
  br label %preloop

afterloop:                                        ; preds = %preloop
  ret i32 0
}
