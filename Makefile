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
RUSTFLAGS=--emit=obj --crate-type=staticlib --target i686-unknown-linux-gnu $(RUSTDEBUG) -C panic=abort -C lto -C target-feature=-mmx,-sse --error-format $(RUSTFORMAT)

OBJ_RUST=kernel.o
OBJ_ASM=boot.o
OBJS=$(OBJ_RUST) $(OBJ_ASM)

.PHONY: all
all: kernel.elf

$(OBJ_ASM): %.o: %.s
	$(AS) $< -o $@ $(ASFLAGS)

$(OBJ_RUST): %.o: %.rs
	$(RUSTC) $< -o $@ $(RUSTFLAGS)

kernel.elf: $(OBJS) linker.lds
	$(LD) -T linker.lds $(OBJS) -o $@

.PHONY: clean
clean:
	$(RM) *.o
	$(RM) *.elf
