import sys, os
from parser import Parser
from codewriter import CodeWriter


def main():
    print("Translator")
    if len(sys.argv) > 1:
        source = sys.argv[1]
        if os.path.isdir(source):
            print("path is dir")
            vm_files = [(source + "/" + f) for f in os.listdir(source) if f.split(".")[-1] == "vm"]
            parsers = [Parser(f) for f in vm_files]
            # [print(len(p.lines)) for p in parsers]
            run(parsers, 0)
        else:
            print("path is file")
            p = Parser(source)
            run(p, 1)
    else:
        print("Please specify filename: => python3 parser.py [filename] ")


def run(parsers, mode):
    if mode == 0: #folder
        c_writer = CodeWriter(parsers[0].filename.split("/")[-2]) #folder_name
        for p in parsers:
            c_writer.set_filename(p.filename.split("/")[-1].split(".")[0]) #each vm_file
            while p.has_more_commands():
                cmd_type = p.command_type()
                if cmd_type == 2 or cmd_type == 3:
                    c_writer.write_push_pop(cmd_type, p.arg1(), p.arg2())
                else:
                    c_writer.write_arithmetic(p._cur()[0])
                p.advance()
    else:
        p = parsers
        c_writer = CodeWriter(p.filename)
        c_writer.set_filename(p.filename.split("/")[-1].split(".")[0])
        while p.has_more_commands():
            cmd_type = p.command_type()
            if cmd_type == 2 or cmd_type == 3:
                c_writer.write_push_pop(cmd_type, p.arg1(), p.arg2())
            else:
                c_writer.write_arithmetic(p._cur()[0])
            p.advance()
    c_writer.close()


if __name__ == "__main__":
    main()
