use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

const VC_KEY: &str = "visitor_count";

#[http_component]
fn handle_spin_test(_req: Request) -> Result<Response> {

    let kv = spin_sdk::key_value::Store::open_default()?;
    let val = match kv.get(VC_KEY) {
        Err(spin_sdk::key_value::Error::NoSuchKey) => 0,
        Err(e) => anyhow::bail!(e),
        Ok(v) => String::from_utf8(v)?.parse()?,
    };
    let count = val + 1;
    kv.set(VC_KEY, count.to_string().as_bytes())?;

    let message = format!("You are visitor number {count}");
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Some(message.into()))?)
}
