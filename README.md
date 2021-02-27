# Introduction

This project is an attempt to write a microkernel with a small environment using mainly Rust but also assembly languages for architecture specific code.

# Prerequisites

* Nightly Rust build

** `rustup toolchain install x86\_64-unknown-linux-gnu`

* make

* binutils

* qemu

# Build

First of all, dependencies have to be built.

```console
rustup component add rust-src
rustc --crate-name core --edition=2018 ~/.rustup/toolchains/<toolchain>/lib/rustlib/src/rust/library/core/src/lib.rs --crate-type rlib --target i686-unknown-none.json
git clone https://github.com/rust-lang/compiler-builtins.git
rustc --crate-name compiler_builtins compiler-builtins/src/lib.rs --crate-type rlib --target i686-unknown-none.json --cfg 'feature="compiler-builtins"' --extern core=libcore.rlib
```

After this two options are available:

1) Debug build: `make`
2) Release build: `DEBUGBUILD=0 make`

# Run

```console
qemu-system-x86_64 -kernel kernel.elf
```
