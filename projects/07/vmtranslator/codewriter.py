import os


class CodeWriter:
    def __init__(self, source: str) -> None:
        super().__init__()
        self.source = source
        self.is_dir = os.path.is_dir(self.source)
        self._output = []
        self.counter = 0

    def set_filename(self, name: str):
        pass

    def write_push_pop(self, cmd: str):
        pass

    def write_arithmetic(self, cmd: int, segment: str, index: int):
        pass

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
