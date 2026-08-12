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
        self
    }
}

impl<T, E> Termination for Result<T, E> {
    fn report(self) -> i32 {
        match self {
            Ok(_) => 0,
            Err(_) => 101,
        }
    }
}
