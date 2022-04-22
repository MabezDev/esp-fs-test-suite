use esp_idf_sys::*;
use c_str_macro::c_str;
use std::{ffi::CStr, fs::File};

fn main() {
    std::thread::sleep(std::time::Duration::from_secs(1));
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    init_partition(c_str!("/storage"), c_str!("storage"), 8);
    std::fs::write("/storage/test_file", "hello").unwrap();

    println!("Testing begins!");

    test_read_to_string();
    test_meta_data();

    println!("Testing complete!");
}

fn test_meta_data() {
    println!("test_meta_data: running");
    let f = File::options().read(true).open("/storage/test_file").unwrap();
    let size = f.metadata().unwrap().len();
    assert_eq!(5, size)
}

fn test_read_to_string() {
    println!("test_read_to_string: running");
    let str = std::fs::read_to_string("/storage/test_file").unwrap();

    println!("String is: {:?}", str);
    assert_eq!("hello", str);
}

fn init_partition(path: &CStr, label: &CStr, max_files: u32) {
    let storage_conf = esp_vfs_spiffs_conf_t {
        base_path: path.as_ptr(),
        partition_label: label.as_ptr(),
        max_files,
        format_if_mount_failed: true,
    };

    unsafe { esp_vfs_spiffs_register(&storage_conf) };
}
