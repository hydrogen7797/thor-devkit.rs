use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use web_sys::{Blob, File};

pub fn blob_to_bytes(blob: Blob) -> Vec<u8> {
    // Create a Uint8Array from the Blob
    let uint8_array = Uint8Array::new(&JsValue::from(blob));

    // Create a Vec<u8> from the Uint8Array
    let mut bytes = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut bytes);

    bytes
}

pub fn file_to_bytes(blob: File) -> Vec<u8> {
    // Create a Uint8Array from the Blob
    let uint8_array = Uint8Array::new(&JsValue::from(blob));

    // Create a Vec<u8> from the Uint8Array
    let mut bytes = vec![0; uint8_array.length() as usize];
    uint8_array.copy_to(&mut bytes);

    bytes
}

pub fn bytes_hash_string(bytes_data: Vec<u8>) -> String {
    let hash = sp_core_hashing::sha2_256(&bytes_data);
    let hash_string: String = hash.iter().map(|byte| format!("{:02x}", byte)).collect();
    hash_string
}

pub fn blob_to_hash_string(blob: Blob) -> String {
    let bytes_data = blob_to_bytes(blob);
    let string_result = bytes_hash_string(bytes_data);
    string_result
}

pub fn file_to_hash_string(blob: File) -> String {
    let bytes_data = file_to_bytes(blob);
    let string_result = bytes_hash_string(bytes_data);
    string_result
}
