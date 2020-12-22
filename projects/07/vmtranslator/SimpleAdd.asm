
//push constant 7
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
//push constant 8
@8
D=A
@SP
A=M
M=D
@SP
M=M+1
//add  
//pop into D  
@SP
A=M
D=M
@SP
M=M-1
//pop into A  
@SP
A=M
D=A
@SP
M=M-1
//add operands D=D+A  
D=D+A
//push D into stack *SP = D  
@SP
A=M
M=D
@SP
M=M+1