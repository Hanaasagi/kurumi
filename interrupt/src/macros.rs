#[macro_export]
macro_rules! scratch_push {
    () => (asm!(
        "push rax
        push rcx
        push rdx
        push rdi
        push rsi
        push r8
        push r9
        push r10
        push r11"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! scratch_pop {
    () => (asm!(
        "pop r11
        pop r10
        pop r9
        pop r8
        pop rsi
        pop rdi
        pop rdx
        pop rcx
        pop rax"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! preserved_push {
    () => (asm!(
        "push rbx
        push rbp
        push r12
        push r13
        push r14
        push r15"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! preserved_pop {
    () => (asm!(
        "pop r15
        pop r14
        pop r13
        pop r12
        pop rbp
        pop rbx"
        : : : : "intel", "volatile"
    ));
}


#[macro_export]
macro_rules! iret {
    () => (asm!(
        "iretq"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! cli {
    () => (asm!(
        "cli"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! sti {
    () => (asm!(
        "sti"
        : : : : "intel", "volatile"
    ));
}

#[macro_export]
macro_rules! interrupt {
    ($name:ident, $body:expr) => {

        #[naked]
        unsafe extern fn $name() {
            #[inline(never)]
            fn inner() {
                $body
            }

            scratch_push!();
            preserved_push!();
            cli!();
            inner();
            sti!();
            preserved_pop!();
            scratch_pop!();
            iret!();

            intrinsics::unreachable();
        }
    };
}
