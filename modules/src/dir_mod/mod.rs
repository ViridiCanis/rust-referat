use std::ops::Mul;

// re-export Modul (als crate::dir_mod::nested)
pub mod nested;

pub fn mul<T: Mul>(x: T, y: T) -> T::Output {
    x * y
}
