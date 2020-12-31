#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;


pub fn it_works() {
    assert_eq!(RS2_PRODUCT_LINE_SR300, 4);
    unsafe { 
        let c_str = rs2_notification_category_to_string(rs2_notification_category_RS2_NOTIFICATION_CATEGORY_FRAMES_TIMEOUT);
        let slice = CStr::from_ptr(c_str);
        println!("string buffer size without nul terminator: {}", slice.to_bytes().len());
        let str_slice: &str = slice.to_str().unwrap();
        println!("string: {}", str_slice);
    };

}
