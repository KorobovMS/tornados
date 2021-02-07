# Introduction

This project is an attempt to write a microkernel with a small environment using mainly Rust but also assembly languages for architecture specific code.

# Prerequisites

* Nightly Rust build
** `rustup toolchain install x86\_64-unknown-linux-gnu`
** `rustup target add i686-unknown-linux-gnu`
* make
* binutils
* qemu

# Build

```console
make
```

# Run

```console
qemu-system-x86_64 -kernel kernel.elf
```
