global start

section .bss
stack_bottom:
    resb 64
stack_top:

section .text
bits 32
start:
    mov esp, stack_top
    call check_cpuid
    call check_long_mode
    ; print `Hello` to screen
    mov WORD [0xb8000], 0x1f48
    mov WORD [0xb8002], 0x1f65
    mov WORD [0xb8004], 0x1f6c
    mov WORD [0xb8006], 0x1f6c
    mov WORD [0xb8008], 0x1f6f
    hlt

error:
    ; print `ERR: code`
    mov WORD [0xb8000], 0x1f45
    mov WORD [0xb8002], 0x1f52
    mov WORD [0xb8004], 0x1f52
    mov WORD [0xb8006], 0x1f3a
    mov WORD [0xb8008], 0x1f20
    mov BYTE [0xb800A], al;
    hlt

check_cpuid:
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21) in
    ; the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
    ; back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit wasn't
    ; flipped, and CPUID isn't supported.
    xor eax, ecx
    jz .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

check_long_mode:
    mov eax, 0x80000000    ; Set the A-register to 0x80000000.
    cpuid                  ; CPU identification.
    cmp eax, 0x80000001    ; Compare the A-register with 0x80000001.
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; Set the A-register to 0x80000001.
    cpuid                  ; CPU identification.
    test edx, 1 << 29      ; Test if the LM-bit, which is bit 29, is set in the D-register.
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error
