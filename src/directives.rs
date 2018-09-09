

#[foo]
struct Foo;

mod bar {
    #![bar]
}

#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}

use std::mem;
#[inline(always)]
fn super_fast_fn() {
    unsafe {
        let a = [0u8, 0u8, 0u8, 0u8];

        let b = mem::transmute::<[u8; 4], u32>(a);
    }
}

#[cfg(target_os = "macos")]
mod macos_only {}

type Hello = String;


