pub fn clamp_down<T: std::ops::Sub<T, Output = T> + std::cmp::PartialOrd<T>>(
    number: T,
    count: T,
) -> T {
    if number >= count {
        number - count
    } else {
        number
    }
}

pub fn clamp_up<T: std::ops::Add<T, Output = T> + std::cmp::PartialOrd<T> + Copy>(
    number: T,
    count: T,
    max: T,
) -> T {
    if number + count <= max {
        number + count
    } else {
        number
    }
}
