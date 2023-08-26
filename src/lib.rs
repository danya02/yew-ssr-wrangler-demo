use worker::*;

pub use console_error_panic_hook::set_once as set_panic_hook;

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
    set_panic_hook();
    Ok(Response::from_html("hello world")?)
}
