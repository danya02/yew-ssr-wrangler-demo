use std::collections::HashMap;

use tokio_stream::StreamExt;
use worker::*;

pub use console_error_panic_hook::set_once as set_panic_hook;

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
    set_panic_hook();
    let url = req.url()?;

    match url.path() {
        "/client_package.js" => {
            let mut js_headers = Headers::new();
            js_headers.append("content-type", "application/javascript")?;
            return Ok(Response::from_body(ResponseBody::Body(
                include_bytes!("../client_package/dist/client_package.js").to_vec(),
            ))?
            .with_headers(js_headers));
        }
        "/client_package_bg.wasm" => {
            let mut wasm_headers = Headers::new();
            wasm_headers.append("content-type", "application/wasm")?;
            return Ok(Response::from_bytes(
                include_bytes!("../client_package/dist/client_package_bg.wasm").to_vec(),
            )?
            .with_headers(wasm_headers));
        }
        _ => (),
    }

    let mut queries = HashMap::new();
    for (k, v) in url.query_pairs() {
        queries.insert(k.to_string(), v.to_string());
    }

    let url = url.path().to_string();

    let renderer =
        yew::ServerRenderer::<yew_app::ServerApp>::with_props(move || yew_app::ServerAppProps {
            url: url.into(),
            queries,
        });

    let page_html = include_str!("../client_package/dist/index.html");
    let (before, after) = page_html.split_once("<body>").unwrap();
    let mut before = before.to_string();
    before.push_str("<body>");
    let after = after.to_string();

    let stream = tokio_stream::once(before)
        .chain(renderer.render_stream())
        .chain(tokio_stream::once(after))
        .map(Result::Ok);

    Ok(Response::from_stream(stream)?)
}
