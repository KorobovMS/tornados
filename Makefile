DEBUGBUILD?=1

AS=as
RUSTC=rustc
LD=ld
RM=rm

ASFLAGS=--32
ifeq ($(DEBUGBUILD), 1)
RUSTDEBUG=-C debuginfo=2 -C opt-level=0
else
RUSTDEBUG=-O
endif

RUST_OPTIONS=--crate-type=rlib --target i686-unknown-none.json $(RUSTDEBUG) -C lto -A dead_code -C panic=abort --edition=2021

OBJ_ASM=boot.o
OBJS_RUST=libkernel.rlib libcompiler_builtins.rlib libcore.rlib

.PHONY: all
all: kernel.elf

libcore.rlib:
	rustc ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/lib.rs --crate-name core $(RUST_OPTIONS)

libcompiler_builtins.rlib: libcore.rlib
	rustc compiler-builtins/src/lib.rs --crate-name compiler_builtins --cfg 'feature="compiler-builtins"' --cfg 'feature="mem"' --extern core=libcore.rlib $(RUST_OPTIONS)

$(OBJ_ASM): %.o: %.s
	$(AS) $< -o $@ $(ASFLAGS)

libkernel.rlib: $(wildcard *.rs) libcore.rlib libcompiler_builtins.rlib
	$(RUSTC) lib.rs --crate-name=kernel --extern core=libcore.rlib --extern compiler_builtins=libcompiler_builtins.rlib $(RUST_OPTIONS)

kernel.elf: $(OBJ_ASM) $(OBJS_RUST) linker.lds
	$(LD) -T linker.lds $(OBJ_ASM) --start-group $(OBJS_RUST) --end-group -o $@ --gc-sections

.PHONY: clean
clean:
	$(RM) *.o
	$(RM) *.rlib
	$(RM) *.elf
