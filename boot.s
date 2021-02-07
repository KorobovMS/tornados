.intel_syntax noprefix

.set ALIGN, 1 << 0
.set MEMINFO, 1 << 1
.set FLAGS, ALIGN | MEMINFO
.set MAGIC, 0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .bss
.align 16
_stack_bottom:
.space 16384
_stack_top:

.section .text
.code32
.global _start
_start:
        lea esp, _stack_top
        mov ebp, esp
        and esp, 0xFFFFFFF0
        call kernel_main
        call hang
.size _start, . - _start

.global hang
hang:
        cli
1:      hlt
        jmp 1b
.size hang, . - hang
