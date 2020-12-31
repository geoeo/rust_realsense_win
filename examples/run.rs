use rust_realsense_win;
use std::ffi::CStr;

fn main() {

    assert_eq!(rust_realsense_win::RS2_PRODUCT_LINE_SR300, 4);
    unsafe { 
        let c_str = rust_realsense_win::rs2_notification_category_to_string(rust_realsense_win::rs2_notification_category_RS2_NOTIFICATION_CATEGORY_FRAMES_TIMEOUT);
        let slice = CStr::from_ptr(c_str);
        println!("string buffer size without nul terminator: {}", slice.to_bytes().len());
        let str_slice: &str = slice.to_str().unwrap();
        println!("string: {}", str_slice);
    };

}
