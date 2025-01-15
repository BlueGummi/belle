# BELLE - The complete program utility set for the Big Endian, Low Level Emulator

[![Linux Build](https://github.com/BlueGummi/belle/actions/workflows/ci.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/ci.yml) 
[![Deploy Astro site to Pages](https://github.com/BlueGummi/belle/actions/workflows/publish.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/publish.yml)
![Demo](http://therealsujitk-vercel-badge.vercel.app/?app=belle-demo)

All documentation is available on [the website for this project](https://bluegummi.github.io/belle) 

## Quickstart

```
git clone https://github.com/BlueGummi/belle --depth=1 && cd belle && ./build.sh -w && ./install.sh -c
```


<details open>
<summary>AUR Installation - Arch Linux and any machine that can install AUR packages</summary>
<br>

```
git clone https://aur.archlinux.org/belle-cpu.git && cd belle-cpu && makepkg -si
```

</details>


Or, for Windows

```pwsh
git clone https://github.com/BlueGummi/belle --depth=1 && cd belle && .\build.ps1 -w && .\install.ps1 -c
```

The binaries can be run by calling `basm`, `belle`, or `bdump`.

## [Further documentation](https://bluegummi.github.io/belle)

### BELLE and the BELLE utilities in action:
[![asciicast](https://asciinema.org/a/697934.svg)](https://asciinema.org/a/697934)

## Naming

**BELLE** is the *emulator*, whilst **BELLE-ISA/ISABELLE** is the *instruction set*.

### Roadmap

Ordered from easy to complex

- [x] Implement error line printing for codegen errors on the assembler

- [x] Make colored disassembling consistent

- [x] Implement address printing for the disassembler

- [x] Implement hex printing for the disassembler

  **ON HOLD** ~~Fix formatter unsafety~~ 

- [ ] Write safe versions of C string functions (for formatter)

- [ ] Finish tutorials

- [ ] Forth-like language + compiler
