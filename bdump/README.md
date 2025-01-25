# bdump - The disassembler for BELLE

### This document is a short overview of the disassembler. For further documentation, visit [docs/bdump.md](https://github.com/BlueGummi/belle/blob/master/docs/bdump.md)

## Quickstart


```make```

To disassemble source code, execute this.

```./bdump main.asm```

Different flags can be passed to make the disassembler emit different output.


| Option               | Description                                         |
|----------------------|-----------------------------------------------------|
| `-h`, `--help`       | Show help message                     |
| `-c`, `--colorless`  | Disable colors                                      |
| `-C`, `--concat-chars`| Concatenate characters                              |
| `-j`, `--no-jump`    | Disable jump visuals                                |
| `-o`, `--only-code`  | Print only disassembled code                        |
| `-b`, `--binary`     | Print instruction binary                            |
| `-X`, `--hex`        | Print instruction operands in hexadecimal           |
| `-V`, `--version`    | Print version                                       |
