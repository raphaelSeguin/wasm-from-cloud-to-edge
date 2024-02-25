//code for calculator lab in Chapter 2

//code without wrapping
// #[no_mangle]
// extern fn add(x: i32, y: i32) -> i32 {
//     x + y
// }
// #[no_mangle]
// extern fn sub(x: i32, y:i32) -> i32 {
//     x - y
// }
// #[no_mangle]
// extern fn mul(x: i32, y: i32) -> i32 {
//     x * y
// }
// #[no_mangle]
// extern fn div(x: i32, y: i32) -> i32 {
//     x / y
// }

//code with wrapping
#[no_mangle]
extern fn add(x: i32, y: i32) -> i32{
    x.wrapping_add(y)
}

#[no_mangle]
extern fn sub(x: i32, y: i32) -> i32 {
    x.wrapping_sub(y)
}

#[no_mangle]
extern fn mul(x: i32, y: i32) -> i32 {
    x.wrapping_mul(y)
}

#[no_mangle]
extern fn div(x: i32, y: i32) -> i32 {
    //performs a checked division
    match x.checked_div(y) {
        Some(result) => result,
        None => 0,
    }
}
