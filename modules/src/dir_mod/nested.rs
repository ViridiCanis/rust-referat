use std::ops::Div;

pub fn div<T: Div>(x: T, y: T) -> T::Output {
    x / y
}
