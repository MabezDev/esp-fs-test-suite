use esp_idf_sys::*;
use c_str_macro::c_str;
use std::ffi::CStr;

fn main() {
    esp_idf_sys::link_patches();

    init_partition(c_str!("/storage"), c_str!("storage"), 3);
    std::fs::write("/storage/test_file", "hello").unwrap();

    test_read_to_string();

    println!("Testing complete!");
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

fn test_read_to_string() {
    let str = std::fs::read_to_string("/storage/test_file").unwrap();

    println!("String is: {:?}", str);
    assert_eq!("hello", str);
}
