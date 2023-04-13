wit_bindgen_rust::export!("wit/spin-http.wit");

struct SpinHttp;

impl spin_http::SpinHttp for SpinHttp {
    fn handle_http_request(_req: spin_http::Request) -> spin_http::Response {
        let body = "This is going over a WIT interface".as_bytes();
        spin_http::Response {
            status: 200,
            headers: None,
            body: Some(body.to_vec()),
        }
    }
}
