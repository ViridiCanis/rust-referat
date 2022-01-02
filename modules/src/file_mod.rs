use std::ops::Sub;

pub fn sub<T: Sub>(x: T, y: T) -> T::Output {
    x - y
}
