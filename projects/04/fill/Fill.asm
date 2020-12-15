// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
    
  
    //get screen  base address
    @SCREEN
    D=A
    // store in the addrscr variable
    @addrscr
    M=D

    //get keyboard address
    @KBD
    D=A
    // store in the addrkbd variable
    @addrkbd
    M=D

    // store counter variable and set to 0
    @counter
    M=0


    @CHECK1
    0;JMP

(WHITE)


(L1)

    @counter
    D=M
    // D=i
    @addrscr
    A=D+M // RAM[addrscr + counter] = 0
    M=0

    @8192
    D=A

    @counter
    M=M+1  // counter = counter + 1
    D=D-M
    @L1
    D;JNE

    // set counter to 0
    @counter
    M=0


 (CHECK1)
    // check (loop) if keyboard is 0, else jump to BLACK
    @addrkbd
    A=M
    D=M 
    @CHECK1
    D;JEQ

    @BLACK
    0;JMP
    


(BLACK)


(L2)

    @counter
    D=M
    // D=i
    @addrscr
    A=D+M // RAM[addrscr + counter] = -1
    M=-1

    @8192
    D=A

    @counter
    M=M+1  // counter = counter + 1
    D=D-M
    @L2
    D;JNE

    // set counter to 0
    @counter
    M=0

 (CHECK2)
    // check (loop) if keyboard is not 0, else jump to WHITE

    @addrkbd
    A=M
    D=M 
    @CHECK2
    D;JNE

    @WHITE
    0;JMP



(END)

    @END
    0;JMP