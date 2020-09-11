; ModuleID = 'main'
source_filename = "main"

@strings = private unnamed_addr constant [6 x i8] c"test\0A\00"

declare i32 @printf(i8*, ...)

define i32 @main() {
entry:
  %ifs = call i32 @ifs()
  ret i32 0
}

define i32 @ifs() {
entry:
  br i32 12, label %then, label %else

then:                                             ; preds = %entry
  %printf = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @strings, i32 0, i32 0))
  br label %end

else:                                             ; preds = %entry
  br label %end

end:                                              ; preds = %else, %then
  ret i32 0
}
