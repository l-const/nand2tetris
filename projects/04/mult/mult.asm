// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.

    // END if RAM[1] == 0 || RAM[0] == 0
    // For R0 times (R0 = RO - 1) 
    // add R1 to R2
    // JEQ R0 = 0 => END

    // Initialize result(RAM[2]) to 0
    @R2
    M=0
    // RAM[1] == 0? => END
    @R1
    D=M      
    @END
    D;JEQ
    // RAM[0] == 0? => END
    @R0
    D=M      
    @END
    D;JEQ    
    
    (LOOP)
    @R0
    D=M             
    D=D-1   // (RAM[0] = RAM[O] - 1) 
    M=D    
    @R1
    D=M    // D = RAM[1]  
    @R2       
    M=M+D   // RAM[2] = RAM[2] + RAM[1]
    @R0
    D=M           
    @LOOP
    D;JNE  // RAM[0] != 0
    (END)
    @END
    0;JMP

