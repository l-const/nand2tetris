import sys, os
from parser import Parser
from codewriter import CodeWriter
from constants import *


def main():
    print("Translator")
    if len(sys.argv) > 1:
        source = sys.argv[1]
        if os.path.isdir(source):
            print("path is dir")
            vm_files = [
                (source + "/" + f)
                for f in os.listdir(source)
                if f.split(".")[-1] == "vm"
            ]
            parsers = [
                Parser(f) for f in vm_files if f.split(".")[-2].split("/")[-1] != "Sys"
            ]
            if source + "/" + "Sys.vm" in vm_files:
                par_sys = Parser(source + "/" + "Sys.vm")
                parsers.append(par_sys)
            run(parsers, 0)
        else:
            print("path is file")
            p = Parser(source)
            run(p, 1)
    else:
        print("Please specify filename: => python3 parser.py [filename] ")


def run(parsers, mode):
    if mode == 0:  # folder
        c_writer = CodeWriter(parsers[0].filename.split("/")[-2])  # folder_name
        for p in parsers:
            if p.filename.split(".")[-2].split("/")[-1] == "Sys":
                print("dddsds")
                c_writer.write_init()
                c_writer.write_call("Sys.init", 0)
                # execute sys.vm parser first
                p_sys = parsers.pop()
                parsers.insert(0, p_sys)
        for p in parsers:
            c_writer.set_filename(
                p.filename.split("/")[-1].split(".")[0]
            )  # each vm_file
            while p.has_more_commands():
                cmd_type = p.command_type()
                if cmd_type == C_PUSH or cmd_type == C_POP:
                    c_writer.write_push_pop(cmd_type, p.arg1(), p.arg2())
                elif cmd_type == C_LABEL:
                    c_writer.write_label(p.arg1())
                elif cmd_type == C_IF:
                    c_writer.write_if(p.arg1())
                elif cmd_type == C_GOTO:
                    c_writer.write_goto(p.arg1())
                elif cmd_type == C_FUNCTION:
                    c_writer.write_function(p.arg1(), p.arg2())
                elif cmd_type == C_RETURN:
                    c_writer.write_return()
                elif cmd_type == C_CALL:
                    c_writer.write_call(p.arg1(), p.arg2())
                else:
                    c_writer.write_arithmetic(p._cur()[0])
                p.advance()
    else:
        p = parsers
        c_writer = CodeWriter(p.filename)
        c_writer.set_filename(p.filename.split("/")[-1].split(".")[0])
        while p.has_more_commands():
            cmd_type = p.command_type()
            # print(f"{cmd_type} {p._cur()}")
            if cmd_type == C_PUSH or cmd_type == C_POP:
                c_writer.write_push_pop(cmd_type, p.arg1(), p.arg2())
            elif cmd_type == C_LABEL:
                c_writer.write_label(p.arg1())
            elif cmd_type == C_IF:
                c_writer.write_if(p.arg1())
            elif cmd_type == C_GOTO:
                c_writer.write_goto(p.arg1())
            elif cmd_type == C_FUNCTION:
                c_writer.write_function(p.arg1(), p.arg2())
            elif cmd_type == C_RETURN:
                c_writer.write_return()
            elif cmd_type == C_CALL:
                c_writer.write_call(p.arg1(), p.arg2())
            else:
                c_writer.write_arithmetic(p._cur()[0])
            p.advance()
    c_writer.close()


if __name__ == "__main__":
    main()
