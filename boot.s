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

.section .data
.global _idt
_idt:
.space 2048

.align 4
.hword 0
.global _idt_ptr
_idt_ptr:
.hword 8*256-1
.long _idt

.section .text
.code32
.global _start
_start:
        lea esp, _stack_top
        mov ebp, esp
        and esp, 0xFFFFFFF0
        lidt _idt_ptr
        call kernel_main
.size _start, . - _start
