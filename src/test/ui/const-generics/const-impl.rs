// run-pass

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

#[allow(dead_code)]

trait Trait {
    fn test(&self) -> usize;
}

impl<T, const N: usize> Trait for [T; N] {
    fn test(&self) -> usize {
        self.len()
    }
}

fn main() {}
