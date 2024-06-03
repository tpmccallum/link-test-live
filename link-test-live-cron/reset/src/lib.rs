use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::{ key_value::Store };

/// A simple Spin HTTP component.
#[http_component]
fn handle_reset(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let store = Store::open_default()?;
    store.set("busy", "no".as_bytes())?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Thanks, you have reset the busy flag.")
        .build())
}
