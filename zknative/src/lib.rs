use std::ffi::{c_char, CString};

mod factorial;

#[no_mangle]
pub extern "C" fn run_triton(number: u64) -> u64 {
    factorial::factorial(number)
}

// #[no_mangle]
// pub extern "C" fn run_triton_with_meta(number: u64) -> &[u8]{
//     let res = factorial::factorial_meta(number);
//     let bytes =serde_json::to_string(&res).unwrap().as_bytes();
//     bytes
// }

#[no_mangle]
pub extern "C" fn run_triton_with_meta(number: u64) -> *mut c_char {
    let res = factorial::factorial_meta(number); // Your logic here
    let json_string = serde_json::to_string(&res).unwrap(); // Convert to JSON string
    let c_string = CString::new(json_string).unwrap(); // Convert to C-compatible string
    c_string.into_raw() // Return a raw pointer to the C string
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            CString::from_raw(s); // Reclaim the memory and allow it to be deallocated
        }
    }
}