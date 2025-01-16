# BELLE - The complete program utility set for the Big Endian, Low Level Emulator

[![Linux Build](https://github.com/BlueGummi/belle/actions/workflows/ci.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/ci.yml) 
[![Deploy Astro site to Pages](https://github.com/BlueGummi/belle/actions/workflows/publish.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/publish.yml)
![Demo](http://therealsujitk-vercel-badge.vercel.app/?app=belle-demo)

All documentation is available on [the website for this project](https://bluegummi.github.io/belle) 

## Quickstart

On **x86 Linux** systems, run

```bash
curl -s https://gist.githubusercontent.com/BlueGummi/beca806b9339c4d934260d1eaf3d84cb/raw/32e89f0e8c4c88867e9faae77d2587e67690d8b6/binstall.sh | bash
```

On **macOS and other Unix systems**, run

```bash
git clone https://github.com/BlueGummi/belle --depth=1 && cd belle && ./build.sh -w && ./install.sh -c
```

<details close>
<summary>AUR Installation - Binary</summary>
<br>

```bash
yay -S belle-cpu
```

</details>


And on Windows, run

```pwsh
iex ((New-Object System.Net.WebClient).DownloadString('https://gist.githubusercontent.com/BlueGummi/7ebc6a09cbf39cc88304cb8a8d8bb571/raw/55918c8bd172e56b33b9ff94a79867a0e6b7ac0d/binstall.ps1'))
```

The binaries can be run by calling `basm`, `belle`, or `bdump`.

## [Further documentation](https://bluegummi.github.io/belle)

### BELLE and the BELLE utilities in action:

[![asciicast](https://asciinema.org/a/698327.svg)](https://asciinema.org/a/698327)


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
