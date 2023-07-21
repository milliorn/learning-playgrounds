#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(1);
}

global_asm! {
    ".global _start",
    "_start:",
    "mov rdi, rsp",
    "call main"
}

fn exit(status: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") status,
            options(noreturn)
        );
    }
}

unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") 1isize => r0,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[no_mangle]
unsafe fn main(_stack_top: *const u8) -> ! {
    write(1, b"Hello world\n".as_ptr(), 12);
    exit(0);
}
