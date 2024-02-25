#[no_mangle]
pub fn add(x: i32, y: i32) -> i32{
    x.wrapping_add(y)
}

#[no_mangle]
pub fn sub(x: i32, y: i32) -> i32 {
    x.wrapping_sub(y)
}

#[no_mangle]
pub fn mul(x: i32, y: i32) -> i32 {
    x.wrapping_mul(y)
}

#[no_mangle]
pub fn div(x: i32, y: i32) -> i32 {
    //performs a checked division
    match x.checked_div(y) {
        Some(result) => result,
        None => 0,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
