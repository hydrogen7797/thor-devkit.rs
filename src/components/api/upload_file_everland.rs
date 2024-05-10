use crate::components::api::select_ipfs_provider::EVERLAND_UPLOAD;

use crate::constants::auth::{EVERLAND_CREDENTIALS, EVERLAND_PIN_TOKEN};
use crate::constants::constant::EVERLAND_ENDPOINT;
use crate::js_extension_binding;

use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, File};

pub async fn upload_everland_file(file: File, name: String) -> String {
    let bucket_endpoint = EVERLAND_ENDPOINT.to_owned();
    let pinning_url = EVERLAND_UPLOAD.to_owned();
    let bucket = "shivarthu-upload".to_owned();
    let folder = "website_tests".to_owned();
    let blob: File = file;
    let file_name = name;
    let file_type = "application/octet-stream".to_owned();
    let access_key = EVERLAND_CREDENTIALS.0.to_owned();
    let secret_access = EVERLAND_CREDENTIALS.1.to_owned();
    let session_token = EVERLAND_CREDENTIALS.2.to_owned();
    let pin_secret = EVERLAND_PIN_TOKEN.to_owned();

    let promise_cid = js_extension_binding::upload_pin_file_4everland(
        bucket_endpoint,
        pinning_url,
        bucket,
        folder,
        blob,
        file_name,
        file_type,
        access_key,
        secret_access,
        session_token,
        pin_secret,
    );
    let js_result = JsFuture::from(promise_cid).await.unwrap();
    let cid = js_result.as_string();
    // log!("data promise", cid.clone());
    cid.unwrap()
}

pub async fn upload_everland_string(json_blob: Blob, name: String) -> String {
    let bucket_endpoint = EVERLAND_ENDPOINT.to_owned();
    let pinning_url = EVERLAND_UPLOAD.to_owned();
    let bucket = "shivarthu-upload".to_owned();
    let folder = "website_tests".to_owned();
    let blob = json_blob;
    let file_name = name;
    let file_type = "text/plain".to_owned();
    let access_key = EVERLAND_CREDENTIALS.0.to_owned();
    let secret_access = EVERLAND_CREDENTIALS.1.to_owned();
    let session_token = EVERLAND_CREDENTIALS.2.to_owned();
    let pin_secret = EVERLAND_PIN_TOKEN.to_owned();

    let promise_cid = js_extension_binding::upload_pin_blob_4everland(
        bucket_endpoint,
        pinning_url,
        bucket,
        folder,
        blob,
        file_name,
        file_type,
        access_key,
        secret_access,
        session_token,
        pin_secret,
    );
    let js_result = JsFuture::from(promise_cid).await.unwrap();
    let cid = js_result.as_string();
    // log!("data promise", cid.clone());
    cid.unwrap()
}
