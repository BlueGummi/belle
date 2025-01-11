# BELLE - The complete program utility set for the Big Endian, Low Level Emulator

[![Linux Build](https://github.com/BlueGummi/belle/actions/workflows/ci.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/ci.yml) 
[![Deploy Astro site to Pages](https://github.com/BlueGummi/belle/actions/workflows/publish.yml/badge.svg)](https://github.com/BlueGummi/belle/actions/workflows/publish.yml)
![Demo](http://therealsujitk-vercel-badge.vercel.app/?app=belle-demo)

All documentation is available on [the website for this project](https://bluegummi.github.io/belle) 

## Quickstart

Cargo, RustC, GCC, and Makefile **compatibility must be present on the system** if these programs are to be installed.

Every executable in this repository can have the `--help` flag passed to display helpful information.


```
git clone https://github.com/BlueGummi/belle && cd belle && ./build.sh && ./install.sh
```

Or, for Windows

```pwsh
git clone https://github.com/BlueGummi/belle && cd belle && .\build.ps1 && .\install.ps1 
```

The binaries can be run by calling `basm`, `belle`, or `bdump`.

## [Further documentation](https://bluegummi.github.io/belle)

### BELLE and the BELLE utilities in action:
[![asciicast](https://asciinema.org/a/697934.svg)](https://asciinema.org/a/697934)

## Naming

**BELLE** is the *emulator*, whilst **BELLE-ISA/ISABELLE** is the *instruction set*.
