
//push constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
//pop local 0
@0
D=A
@1
A=D+M
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
//label LOOP_START 
(BasicLoop.$LOOP_START)
//push argument 0
@0
D=A
@2
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
//push local 0
@0
D=A
@1
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
//add  
@SP
AM=M-1
D=M
A=A-1
M=M+D
//pop local 0
@0
D=A
@1
A=D+M
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
//push argument 0
@0
D=A
@2
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
//push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
//sub  
@SP
AM=M-1
D=M
A=A-1
M=M-D
//pop argument 0
@0
D=A
@2
A=D+M
D=A
@R13
M=D
@SP
AM=M-1
D=M
@R13
A=M
M=D
//push argument 0
@0
D=A
@2
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
//if-goto LOOP_START 
@SP
AM=M-1
D=M
@BasicLoop.$LOOP_START
D;JNE
//push local 0
@0
D=A
@1
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1