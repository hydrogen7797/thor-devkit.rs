use gloo::net::http::{Request, Response};

pub async fn ipfs_fetch_response(hash: &str, ipfs_address: &str) -> Response {
    let resp = Request::get(&format!("{}{}", ipfs_address, hash))
        .send()
        .await
        .unwrap();
    resp
}
