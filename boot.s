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

.align 16
_tss_stack_bottom:
.space 16384
_tss_stack_top:

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
.byte 0xCF /* G(1) | D/B(1) | L(0) | AVL(0) | Limit(1111) */
.byte 0x0 /* Base */

/* Kernel data segment descriptor */
.hword 0xFFFF /* Limit */
.hword 0x0 /* Base */
.byte 0x0 /* Base */
.byte 0x92 /* P(1) | DPL(00) | S(1) | Type(0010) */
.byte 0xCF /* G(1) | D/B(1) | L(0) | AVL(0) | Limit(1111) */
.byte 0x0 /* Base */

/* User code segment descriptor */
.hword 0xFFFF /* Limit */
.hword 0x0 /* Base */
.byte 0x0 /* Base */
.byte 0xFA /* P(1) | DPL(11) | S(1) | Type(1010) */
.byte 0xCF /* G(1) | D/B(1) | L(0) | AVL(0) | Limit(1111) */
.byte 0x0 /* Base */

/* User data segment descriptor */
.hword 0xFFFF /* Limit */
.hword 0x0 /* Base */
.byte 0x0 /* Base */
.byte 0xF2 /* P(1) | DPL(11) | S(1) | Type(0010) */
.byte 0xCF /* G(1) | D/B(1) | L(0) | AVL(0) | Limit(1111) */
.byte 0x0 /* Base */

/* TSS segment descriptor */
_tss_desc:
.hword 0x67 /* Limit */
.hword 0x0 /* Base */
.byte 0x0 /* Base */
.byte 0x89 /* P(1) | DPL(00) | 0 | Type(1001) */
.byte 0x0 /* G(0) | 0 | 0 | AVL(0) | Limit(0000) */
.byte 0x0 /* Base */

.set KERNEL_CS, 0x8 /* 1 index | GDT | 0 RPL */
.set KERNEL_DS, 0x10 /* 2 index | GDT | 0 RPL */
.set USER_CS, 0x1B /* 3 index | GDT | 3 RPL */
.set USER_DS, 0x23 /* 4 index | GDT | 3 RPL */
.set TSS_S, 0x28 /* 5 index | GDT | 0 RPL */

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
.hword 8*6-1
.long _gdt

.align 16
_tss:
.long 0
.long _tss_stack_top
.hword KERNEL_DS
.hword 0
.fill 23, 4, 0

.global _multiboot_info
_multiboot_info:
.long 0

.section .text
.code32
.global _start
_start:
        cli
        cld

        mov _multiboot_info, ebx

        /* setting GDT */
        lgdt _gdt_ptr

        /* setting all selectors */
        jmp KERNEL_CS:_new_cs
_new_cs:
        mov ax, KERNEL_DS
        mov ds, ax
        mov ss, ax
        mov es, ax
        mov fs, ax
        mov gs, ax

        /* setting TSS ptr in TSS descriptor in GDT */
        lea ebp, _tss_desc
        lea eax, _tss
        mov [ebp + 2], ax
        shr eax, 16
        mov [ebp + 4], al
        mov [ebp + 7], ah

        /* setting Task Register with TSS selector */
        mov ax, TSS_S
        ltr ax

        /* setting initial kernel stack */
        lea esp, _stack_top
        mov ebp, esp
        and esp, 0xFFFFFFF0

        /* setting IDT */
        lidt _idt_ptr

        /* go to Rust code */
        call kernel_main
.size _start, . - _start

.global restore_thread
restore_thread:
        mov eax, [esp + 4*11]
        test eax, 3
        jz _restore_kernel_thread
_restore_user_thread:
        mov eax, [esp + 4*0 + 4*12]
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        push eax /* ss */
        push [esp + 4*1 + 4*8] /* esp */
        push [esp + 4*2 + 4*10] /* eflags */
        push [esp + 4*3 + 4*11] /* cs */
        push [esp + 4*4 + 4*9] /* eip */
        mov eax, [esp + 4*5 + 4*1]
        mov ebx, [esp + 4*5 + 4*2]
        mov ecx, [esp + 4*5 + 4*3]
        mov edx, [esp + 4*5 + 4*4]
        mov esi, [esp + 4*5 + 4*5]
        mov edi, [esp + 4*5 + 4*6]
        mov ebp, [esp + 4*5 + 4*7]
        iretd
_restore_kernel_thread:
        mov ebp, [esp + 4*8]
        mov eax, [esp + 4*10]
        mov [ebp - 4], eax
        mov eax, [esp + 4*11]
        mov [ebp - 8], eax
        mov eax, [esp + 4*9]
        mov [ebp - 12], eax
        mov eax, [esp + 4*1]
        mov ebx, [esp + 4*2]
        mov ecx, [esp + 4*3]
        mov edx, [esp + 4*4]
        mov esi, [esp + 4*5]
        mov edi, [esp + 4*6]
        mov ebp, [esp + 4*7]
        mov esp, [esp + 4*8]
        sub esp, 12
        iretd
.size restore_thread, . - restore_thread
