import os
from constants import *

class CodeWriter:

    cmd_dict = {
        1: "arithmetic",
        2: "push",
        3: "pop",
        4: "label",
        5: "goto",
        6: "if",
        7: "function",
        8: "return",
        9: "call",
    }

    def __init__(self, source: str) -> None:
        super().__init__()
        self.source = source
        self.vm_file = source 
        self.is_dir = os.path.isdir(self.source)
        self._output = []
        self.out_file = open(self._get_out_file(), "w+")
        self.counter = 0
        self.label_flag = 0

    def set_filename(self, name: str):
        self.vm_file = name

    def _get_sp(self, offset=""):  # @SP
        self.out_file.write(f"\n@SP\nA=M{offset}")

    def inc_sp(self):  # SP++
        self.out_file.write("\n@SP\nM=M+1")

    def dec_sp(self):  # SP--
        self.out_file.write("\n@SP\nM=M-1")

    def get_constant(self, index: int):  # D = index
        self.out_file.write(f"\n@{index}\nD=A")
    
    def get_local(self, index:int ,kind=0):
        if not kind:
            self.out_file.write(f"\n@{index}\nD=A\n@{LCL}\nA=D+M\nD=M")
        else:
            self.out_file.write(f"\n@{index}\nD=A\n@{LCL}\nA=D+M\nD=A")
            
    def get_arguments(self, index:int, kind=0):
        if not kind:
            self.out_file.write(f"\n@{index}\nD=A\n@{ARG}\nA=D+M\nD=M")
        else:
            self.out_file.write(f"\n@{index}\nD=A\n@{ARG}\nA=D+M\nD=A")
            
    def get_this(self, index:int, kind=0):
        if not kind:
            self.out_file.write(f"\n@{index}\nD=A\n@{THIS}\nA=D+M\nD=M")
        else:
            self.out_file.write(f"\n@{index}\nD=A\n@{THIS}\nA=D+M\nD=A")

    def get_that(self, index:int, kind=0):
        if not kind:
            self.out_file.write(f"\n@{index}\nD=A\n@{THAT}\nA=D+M\nD=M")
        else:
            self.out_file.write(f"\n@{index}\nD=A\n@{THAT}\nA=D+M\nD=A")

    def get_temp(self, index:int, kind=0):
        # temp segment start at ram[5]
        if not kind:
            self.out_file.write(f"\n@{5+index}\nD=M")
        else:
            self.out_file.write(f"\n@{5+index}\nD=A")

    def get_pointer(self, index:int, kind=0):
        # pointer  segment start at ram[3]
        if not kind:
            self.out_file.write(f"\n@{3+index}\nD=M")
        else:
            self.out_file.write(f"\n@{3+index}\nD=A")

    def get_static(self, index:int, kind=0):
        # static  segment start at ram[16]
        if not kind:
            self.out_file.write(f"\n@{16+index}\nD=M")
        else:
            self.out_file.write(f"\n@{16+index}\nD=A")
   
    def push(self, reg="D"):  # *SP=D,A , SP++
        self._get_sp()      #A=M
        self.out_file.write(f"\nM={reg}")  # M = D, M = A
        self.inc_sp()

    def write_comment(self, cmd, segment=None, index=None):        
        if segment is None:
            segment = ""
        if index is None:
            index = ""
        self.out_file.write(f"\n//{cmd} {segment} {index}")

    def pop(self, reg="M"):  # *D,A = *SP , SP--
        self.out_file.write("\nAM=M-1")
        self.out_file.write(f"\nD={reg}")  # D-M, D=A

    def binary(self, cmd: str):
        if cmd == "add":
            self.translate_add()
        elif cmd == "sub":
            self.translate_sub()
        elif cmd == "and":
            self.translate_and()
        elif cmd == "or":
            self.translate_or()
        elif cmd == "eq":
            self.translate_logic("JNE") #OPPOSITE
        elif cmd == "gt":
            self.translate_logic("JLE") #OPPOSITE
        else:
            self.translate_logic("JGE") #OPPOSITE

    def unary(self, cmd: str):
        if cmd == "not":
            self.translate_not()
        else:
            self.translate_neg()
    
    def r13_comp(self, comp:str):
        self.out_file.write(f"\n@R13")   
        self.out_file.write(f"\n{comp}")     
            
   
    def translate_arithm(self, oper: str):
        self.out_file.write(f"\n@SP")
        self.out_file.write(f"\nAM=M-1")     
        self.out_file.write(f"\nD=M")     
        self.out_file.write(f"\nA=A-1")     
        self.out_file.write(f"\n{oper}")


    def translate_add(self):
        self.translate_arithm("M=M+D")

    def translate_sub(self):
        self.translate_arithm("M=M-D")
    
    def translate_or(self):
        self.translate_arithm("M=M|D")
    
    def translate_and(self):
        self.translate_arithm("M=M&D")
    
    def translate_logic(self, type:str):
        self.out_file.write (
            f"""\n@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D""" +   
            f"""\n@FALSE{self.label_flag}""" +
            f"""\nD;{type}\n@SP\nA=M-1\nM=-1""" + 
            f"""\n@CONTINUE{self.label_flag} """ + 
            f"""\n0;JMP""" + 
            f"""\n(FALSE{self.label_flag})""" +   
            f"""\n@SP""" +  
            f"""\nA=M-1""" +  
            f"""\nM=0"""   + 
            f"""\n(CONTINUE{self.label_flag})"""
        )
        self.label_flag +=1
    
    def translate_not(self):
        self.out_file.write(f"\n@SP")
        self.out_file.write(f"\nA=M-1")
        self.out_file.write(f"\nM=!M")

    def translate_neg(self):
        self.out_file.write(f"\nD=0")
        self.out_file.write(f"\n@SP")
        self.out_file.write(f"\nA=M-1")
        self.out_file.write(f"\nM=D-M")
    
    def write_push_pop(self, cmd: int, segment: str, index: int):
        self.write_comment(CodeWriter.cmd_dict[cmd], segment, index)
        if cmd == 2: #push
            if segment == "constant":
                self.get_constant(index)
                self.push()
            elif segment == "temp":
                self.get_temp(index)
                self.push()
            elif segment == "argument":
                self.get_arguments(index)
                self.push()
            elif segment == "local":
                self.get_local(index)
                self.push()
            elif segment == "this":
                self.get_this(index)
                self.push()
            elif segment == "that":
                self.get_that(index)
                self.push()
            elif segment == "static":
                self.get_static(index)
                self.push()
            elif segment == "pointer":
                self.get_pointer(index)
                self.push()
            else:
                print("push() else")

        else: #pop
            
            if segment == "temp":
                self.get_temp(index, kind=1)
                self.out_file.write(f"\n@R13\nM=D")
                self.out_file.write(f"\n@SP")
                self.pop()
                self.out_file.write(f"\n@R13\nA=M\nM=D")
                
            elif segment == "argument":
                self.get_arguments(index, kind=1)
                self.out_file.write(f"\n@R13\nM=D")
                self.out_file.write(f"\n@SP")
                self.pop()
                self.out_file.write(f"\n@R13\nA=M\nM=D")

            elif segment == "local":
                self.get_local(index, kind=1)
                self.out_file.write(f"\n@R13\nM=D")
                self.out_file.write(f"\n@SP")
                self.pop()
                self.out_file.write(f"\n@R13\nA=M\nM=D")

            elif segment == "this":
                self.get_this(index, kind=1)
                self.out_file.write(f"\n@R13\nM=D")
                self.out_file.write(f"\n@SP")
                self.pop()
                self.out_file.write(f"\n@R13\nA=M\nM=D")
                
            elif segment == "that":
                self.get_that(index, kind=1)
                self.out_file.write(f"\n@R13\nM=D")
                self.out_file.write(f"\n@SP")
                self.pop()
                self.out_file.write(f"\n@R13\nA=M\nM=D")

            elif segment == "static":
                self.get_static(index, kind=1)
                self.out_file.write(f"\n@R13\nM=D")
                self.out_file.write(f"\n@SP")
                self.pop()
                self.out_file.write(f"\n@R13\nA=M\nM=D")

            elif segment == "pointer":
                self.get_pointer(index, kind=1)
                self.out_file.write(f"\n@R13\nM=D")
                self.out_file.write(f"\n@SP")
                self.pop()
                self.out_file.write(f"\n@R13\nA=M\nM=D")

            else:
                print("pop() else")
                
            
    def write_arithmetic(self, cmd: str):
        self.write_comment(cmd)
        if cmd != "not" and cmd != "neg":
            self.binary(cmd)
        else:
            self.unary(cmd)

    def _get_out_file(self):
        if self.is_dir:
            out_file = self.source + ".asm"
        else:
            out_file = self.source.replace(".vm", ".asm")
        return out_file

    def close(self):
        self.out_file.close()
