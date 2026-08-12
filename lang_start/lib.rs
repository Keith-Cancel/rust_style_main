#![no_std]
#![feature(lang_items, never_type)]
#![allow(internal_features)]

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
