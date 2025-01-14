# BELLE - The complete program utility set for the Big Endian, Low Level Emulator

[![Linux Build](https://github.com/BlueGummi/belle/actions/workflows/ci.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/ci.yml) 
[![Deploy Astro site to Pages](https://github.com/BlueGummi/belle/actions/workflows/publish.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/publish.yml)
![Demo](http://therealsujitk-vercel-badge.vercel.app/?app=belle-demo)

All documentation is available on [the website for this project](https://bluegummi.github.io/belle) 

## Quickstart

```
git clone https://github.com/BlueGummi/belle --depth=1 && cd belle && ./build.sh && ./install.sh
```

Or, for Windows

```pwsh
git clone https://github.com/BlueGummi/belle --depth=1 && cd belle && .\build.ps1 && .\install.ps1 
```

The binaries can be run by calling `basm`, `belle`, or `bdump`.

## [Further documentation](https://bluegummi.github.io/belle)

### BELLE and the BELLE utilities in action:
[![asciicast](https://asciinema.org/a/697934.svg)](https://asciinema.org/a/697934)

## Naming

**BELLE** is the *emulator*, whilst **BELLE-ISA/ISABELLE** is the *instruction set*.

### Roadmap

Ordered from easy to complex

- [ ] Implement error line printing for codegen errors on the assembler

- [ ] Make colored disassembling consistent

- [ ] Implement address printing for the disassembler

- [ ] Implement hex printing for the disassembler

- [ ] Fix formatter unsafety

- [ ] Finish tutorials

- [ ] Forth-like language + compiler
