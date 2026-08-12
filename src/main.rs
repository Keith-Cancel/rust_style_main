#![no_std]

extern crate lang_start;

fn main() {
    stdout(b"Hello: from main\n");
}

// ==== Items needed for no_std ====

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(101);
}

// Switch about to custom ABI once this is stabilized:
// https://github.com/rust-lang/rust/pull/158504
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    core::arch::naked_asm! {
        "xor rbp, rbp",  // clear frame register
        "mov rdi, rsp",  // copy stack pointer to arg1,
        "and rsp, -16",  // align stack to 16 bytes
        "call __premain",
        "ud2"
    }
}

#[unsafe(no_mangle)]
extern "C" fn __premain(stk: *mut core::ffi::c_void) -> ! {
    stdout(b"In function `__premain`\n");
    // Omitting setting up argc and argv from the stack.
    // but these would be passed to main.
    let _ = stk;
    // Get the pointer to the generated rustc main, and call it.
    let ret = unsafe { lang_start::get_rustc_main()(0, core::ptr::null()) };
    exit(ret);
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
