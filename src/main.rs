#![no_std]

fn main() {
    stdout(b"Hello: from main\n");
}

// ==== Items needed for no_std ====

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(101);
}

// ==== Some basic linux sys calls ====

fn exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rdi") code,
            in("rax") 60,
            options(preserves_flags, nostack, noreturn),
        );
    }
}

fn stdout(buff: &[u8]) -> isize {
    let mut ret: isize = 1;
    let ptr = buff.as_ptr();
    let len = buff.len();
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rdi") 0,
            in("rsi") ptr,
            in("rdx") len,
            inout("rax") ret,
            out("r11") _, // syscall clobbers both rcx and r11
            out("rcx") _,
            options(preserves_flags, nostack),
        );
    }
    return ret;
}
