// alles was hier als pub markiert ist, ist von außen sichtbar
// z.B. in main.rs oder wenn jemand dieses Crate verwendet als Dependency

// alle Module müssen erwähnt werden
// crate::file_mod (hat nested)
mod file_mod;
// crate::dir_mod
pub mod dir_mod;

// re-export Modulinhalt
pub use file_mod::sub;

use std::ops::Add;

pub fn add<T: Add>(x: T, y: T) -> T::Output {
    x + y
}

pub mod inner {
    use std::ops::Mul;

    pub fn square<T: Mul + Clone>(x: T) -> T::Output {
        crate::dir_mod::mul(x.clone(), x)
    }
}

/*
alles was von außen sichtbar ist:

fn add
fn sub
mod dir_mod:
    fn mul
    mod nested:
        fn div
mod inner:
    fn square
*/
