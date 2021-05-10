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

.global _gdt
_gdt:
/* Zero descriptor */
.quad 0

/* Kernel code segment descriptor */
.hword 0xFFFF /* Limit */
.hword 0x0 /* Base */
.byte 0x0 /* Base */
.byte 0x9A /* P(1) | DPL(00) | S(1) | Type(1010) */
.byte 0xCF /* G(1) | D/B(1) | L(1) | Limit(4) */
.byte 0x0 /* Base */

/* Kernel data segment descriptor */
.hword 0xFFFF /* Limit */
.hword 0x0 /* Base */
.byte 0x0 /* Base */
.byte 0x92 /* P(1) | DPL(00) | S(1) | Type(0010) */
.byte 0xCF /* G(1) | D/B(1) | L(1) | Limit(4) */
.byte 0x0 /* Base */

/* Other descriptors */
.space 2024

.align 4
.hword 0
.global _idt_ptr
_idt_ptr:
.hword 8*256-1
.long _idt

.align 4
.hword 0
.global _gdt_ptr
_gdt_ptr:
.hword 8*256-1
.long _gdt

.set KERNEL_CS, 0x8 /* 1 index | GDT | 0 PL */
.set KERNEL_DS, 0x10 /* 2 index | GDT | 0 PL */

.section .text
.code32
.global _start
_start:
        lea esp, _stack_top
        mov ebp, esp
        and esp, 0xFFFFFFF0
        lgdt _gdt_ptr
        jmp KERNEL_CS:_new_cs
_new_cs:
        mov ax, KERNEL_DS
        mov ds, ax
        mov ss, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        lidt _idt_ptr
        call kernel_main
.size _start, . - _start
