import os


class CodeWriter:
    def __init__(self, source: str) -> None:
        super().__init__()
        self.source = source
        self.is_dir = os.path.isdir(self.source)
        self._output = []
        self.counter = 0

    def set_filename(self, name: str):
        pass

    def write_push_pop(self, cmd: str,  segment: str, index: int):
        out = []
        out.append(f"//{cmd} {segment} {index}")
        self._output.append(out.pop())
        if segment == "constant" and cmd == 2:
            out.append(f"@{index}\nD=M")
            out.append(f"@SP\nA=M")
            out.append(f"M=D")
            out.append(f"@SP\nM=M+1")
            
            [self._output.append(o) for o in out]
        if cmd == 3:
            out.append(f"@SP\nA=M")
            out.append(f"D=M")
            out.append(f"@SP\nM=M-1")
            
            [self._output.append(o) for o in out]

    def write_arithmetic(self, cmd: int):
        out = []
        out.append(f"//{cmd}")
        [self._output.append(o) for o in out]
         
        self.write_push_pop("pop", "", 0)

    def close(self):
        if not self.is_dir:
            name = self.source.split(".")[0]
        else:
            name = self.source
        out_file = name + ".asm"
        with open(out_file, "w+") as out_f:
            out_f.writelines(
                ["\n" + l if p != 0 else l for p, l in enumerate(self._output)]
            )
