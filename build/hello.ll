; ModuleID = 'main'
source_filename = "main"

declare i32 @putchar(i32)

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
  %putchar6 = call i32 @putchar(i32 119)
  %putchar7 = call i32 @putchar(i32 111)
  %putchar8 = call i32 @putchar(i32 114)
  %putchar9 = call i32 @putchar(i32 108)
  %putchar10 = call i32 @putchar(i32 100)
  %putchar11 = call i32 @putchar(i32 10)
  br label %end

end:                                              ; preds = %else, %then
  ret i32 0
}

define i32 @ifs.1() {
entry:
  br i1 false, label %then, label %else

then:                                             ; preds = %entry
  %putchar = call i32 @putchar(i32 104)
  %putchar1 = call i32 @putchar(i32 101)
  %putchar2 = call i32 @putchar(i32 108)
  %putchar3 = call i32 @putchar(i32 108)
  %putchar4 = call i32 @putchar(i32 111)
  %putchar5 = call i32 @putchar(i32 10)
  br label %end

else:                                             ; preds = %entry
  %putchar6 = call i32 @putchar(i32 119)
  %putchar7 = call i32 @putchar(i32 111)
  %putchar8 = call i32 @putchar(i32 114)
  %putchar9 = call i32 @putchar(i32 108)
  %putchar10 = call i32 @putchar(i32 100)
  %putchar11 = call i32 @putchar(i32 10)
  br label %end

end:                                              ; preds = %else, %then
  ret i32 0
}

define i32 @main() {
entry:
  %putchar = call i32 @putchar(i32 105)
  %putchar1 = call i32 @putchar(i32 51)
  %putchar2 = call i32 @putchar(i32 50)
  %putchar3 = call i32 @putchar(i32 32)
  %putchar4 = call i32 @putchar(i32 49)
  %putchar5 = call i32 @putchar(i32 52)
  %putchar6 = call i32 @putchar(i32 10)
  %ifs = call i32 @ifs()
  %ifs7 = call i32 @ifs.1()
  ret i32 0
}
