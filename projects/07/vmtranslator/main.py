import sys, os
from parser import Parser


def main():
    print("Translator")
    if len(sys.argv) > 1:
        source = sys.argv[1]
        if os.path.isdir(source):
            print("path is dir")
            vm_files = [(source + "/" + f) for f in os.listdir(source)]
            parsers = [Parser(f) for f in vm_files]
            # [print(len(p.lines)) for p in parsers]
        else:
            print("path is file")

    else:
        print("Please specify filename: => python3 parser.py [filename] ")


if __name__ == "__main__":
    main()
