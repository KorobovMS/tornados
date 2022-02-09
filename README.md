# Introduction

This project is an attempt to write a kernel with a small environment using mainly Rust but also assembly languages for architecture specific code.

# Prerequisites

* Nightly Rust build
    * `rustup toolchain install x86_64-unknown-linux-gnu`
* make
* binutils
* qemu

# Build

First of all, dependencies have to be built.

```console
rustup component add rust-src
rustc --crate-name core --edition=2021 ~/.rustup/toolchains/<toolchain>/lib/rustlib/src/rust/library/core/src/lib.rs --crate-type rlib --target i686-unknown-none.json
git clone https://github.com/rust-lang/compiler-builtins.git
rustc --crate-name compiler_builtins compiler-builtins/src/lib.rs --crate-type rlib --target i686-unknown-none.json --cfg 'feature="compiler-builtins"' --extern core=libcore.rlib
```

After this two options are available:

1) Debug build: `make`
2) Release build: `DEBUGBUILD=0 make`

# Run

The simplest way to run this kernel is using the Qemu. `-kernel` argument allows you to launch multiboot 0.6.96 compatible kernel.

```console
qemu-system-i386 -kernel kernel.elf -serial stdio
```

# Run with GRUB

The other way is to use a multiboot-compliant bootloader (like GRUB) which can load this kernel from disk.

1) Create a virtual disk. For example:

```console
dd if=/dev/zero of=hd.img bs=512 count=60480
```

2) Create partition table, partition and make it bootable

```console
fdisk hd.img
# o: create DOS partition table
# n: create partition
# a: make it bootable
# w: save scheme to virtual disk
```

3) Make loop device for disk image

```console
losetup -f
losetup -P /dev/loop0 ./hd.img
```

4) Make FAT FS in the first partition

```console
mkfs.vfat /dev/loop0p1
```

5) Fill FS with files

```console
mount -o loop /dev/loop0p1 /mnt
mkdir -p /mnt/boot/grub
vi /mnt/boot/grub/grub.cfg
# Example of grub.cfg:
# menuentry "tornados" {
#     insmod fat
#     insmod normal
#     insmod multiboot
#     set root=(hd0,msdos1)
#     multiboot /kernel.elf
#     boot
# }
cp kernel.elf /mnt/
```

6) Install GRUB

```console
grub-install --target=i386-pc --boot-directory=/mnt/boot/ --modules="multiboot fat part_msdos" /dev/loop0
```

7) Synchronize FS with virtual disk

```console
sync
```

Now it's possible to run this kernel with:

```console
qemu-system-i386 -serial stdio -hda ./hd.img
```

The same idea applies for running this kernel with real hardware.
