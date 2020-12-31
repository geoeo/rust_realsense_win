extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {

    println!("cargo:rustc-link-lib=realsense2");
    println!("cargo:rustc-link-search=C:/Program Files (x86)/Intel RealSense SDK 2.0/lib/x64");
    println!("cargo:rustc-env=PATH=C:/Program Files (x86)/Intel RealSense SDK 2.0/bin/x64");

    let bindings = bindgen::builder()
    .clang_arg("-fno-inline-functions")
    .clang_arg("-IC:/Program Files (x86)/Intel RealSense SDK 2.0/include/librealsense2")
    .header("rs.h")
    .header("rs_advanced_mode.h")
    .header("rsutil.h")
    .header("h/rs_advanced_mode_command.h")
    .header("h/rs_config.h")
    .header("h/rs_context.h")
    .header("h/rs_device.h")
    .header("h/rs_frame.h")
    .header("h/rs_option.h")
    .header("h/rs_pipeline.h")
    .header("h/rs_processing.h")
    .header("h/rs_record_playback.h")
    .header("h/rs_sensor.h")
    .header("C:/Program Files (x86)/Intel RealSense SDK 2.0/include/librealsense2/h/rs_types.h")
    .whitelist_var("RS2_.*")
    .whitelist_type("rs2_.*")
    .whitelist_function("rs2_.*")
    .whitelist_function("_rs2_.*")
    .generate()
    .expect("Couldn't generate bindings!");



    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}