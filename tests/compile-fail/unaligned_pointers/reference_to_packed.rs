// This should fail even without validation/SB
// compile-flags: -Zmiri-disable-validation -Zmiri-disable-stacked-borrows

#![allow(dead_code, unused_variables, unaligned_references)]

#[repr(packed)]
struct Foo {
    x: i32,
    y: i32,
}

fn main() {
    for _ in 0..10 { // Try many times as this might work by chance.
        let foo = Foo {
            x: 42,
            y: 99,
        };
        let p = &foo.x;
        let i = *p; //~ERROR alignment ALIGN is required
    }
}
