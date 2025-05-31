use std::ops::Add;

pub fn add<T: Add<T, Output = T>>(left: T, right: T) -> T {
    left + right 
}

