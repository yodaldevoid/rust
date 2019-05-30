// run-pass

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete and may cause the compiler to crash

pub fn function_with_str<const STRING: &'static str>() {}

pub fn main() {
    function_with_str::<"Hello, world!">()
}
