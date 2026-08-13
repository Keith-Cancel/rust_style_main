#![no_std]
#![feature(lang_items, never_type)]
#![allow(internal_features)]

/// rustc's own compiler generated `main` calls this
#[lang = "start"]
pub fn my_start<T: Termination + 'static>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _sigpipe: u8,
) -> isize {
    stdout(b"In function: `my_start`\n\n");
    main().report() as isize
}

// rustc generates a main function that uses our start function.
#[inline(never)]
pub fn get_rustc_main() -> unsafe extern "C" fn(isize, *const *const u8) -> i32 {
    unsafe extern "C" {
        #[link_name = "main"]
        fn rustc_main(argc: isize, argv: *const *const u8) -> i32;
    }
    return rustc_main;
}

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        stdout(b"\nIn Termination::report() for: `()`\n");
        0
    }
}

impl Termination for ! {
    fn report(self) -> i32 {
        unreachable!()
    }
}

impl Termination for i32 {
    fn report(self) -> i32 {
        stdout(b"\nIn Termination::report() for: `i32`\n");
        self
    }
}

impl<T, E> Termination for Result<T, E> {
    fn report(self) -> i32 {
        stdout(b"\nIn Termination::report() for: `Result<_,_>`\n");
        match self {
            Ok(_) => 0,
            Err(_) => 101,
        }
    }
}

fn stdout(buff: &[u8]) -> isize {
    let mut ret: isize = 1;
    let ptr = buff.as_ptr();
    let len = buff.len();
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rdi") 1,
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
