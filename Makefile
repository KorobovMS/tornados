DEBUGBUILD?=1
RUSTFORMAT?=short

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
RUSTFLAGS=--crate-type=rlib --target i686-unknown-none.json $(RUSTDEBUG) -C lto --error-format $(RUSTFORMAT) --extern core=libcore.rlib --extern compiler_builtins=libcompiler_builtins.rlib

OBJ_ASM=boot.o
OBJS_RUST=libkernel.rlib libcompiler_builtins.rlib libcore.rlib

.PHONY: all
all: kernel.elf

$(OBJ_ASM): %.o: %.s
	$(AS) $< -o $@ $(ASFLAGS)

libkernel.rlib: $(wildcard *.rs)
	$(RUSTC) lib.rs -o $@ $(RUSTFLAGS)

kernel.elf: $(OBJ_ASM) $(OBJS_RUST) linker.lds
	$(LD) -T linker.lds $(OBJ_ASM) --start-group $(OBJS_RUST) --end-group -o $@ --gc-sections

.PHONY: clean
clean:
	$(RM) *.o
	$(RM) *.elf
	$(RM) libkernel.rlib
