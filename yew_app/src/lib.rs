use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[cfg(feature = "csr")]
use yew::suspense::use_future;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/page1")]
    Page1,
    #[at("/page2")]
    Page2,
    #[at("/page3")]
    Page3,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

// !!! README !!!
// Below, the ServerApp and App must contain exactly the same HTML:
// If you want to add a component outside the Router or the Switch,
// make sure to apply the change to both ServerApp and App!

#[function_component]
#[allow(non_snake_case)]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = MemoryHistory::new();

    log::info!("Props received during rendering: {props:?}");

    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    let history = AnyHistory::from(history);

    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}

#[function_component]
#[allow(non_snake_case)]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct HttpbinIpResponse {
    pub origin: String,
}

#[cfg(feature = "ssr")]
async fn get_server_ip() -> HttpbinIpResponse {
    let mut resp = worker::Fetch::Url("https://httpbin.org/ip".parse().unwrap())
        .send()
        .await
        .unwrap();

    resp.json().await.unwrap()
}

#[function_component]
#[allow(non_snake_case)]
fn ServerIpInner() -> HtmlResult {
    let server_ip = use_prepared_state!(
        async move |_| -> HttpbinIpResponse { get_server_ip().await },
        ()
    )?
    .unwrap();
    Ok(html!({ &server_ip.origin }))
}

#[function_component]
#[allow(non_snake_case)]
fn ServerIp() -> Html {
    let fallback = html!({ "loading..." });
    html!(
        <Suspense {fallback}>
            <ServerIpInner />
        </Suspense>
    )
}

#[function_component]
#[allow(non_snake_case)]
fn ClientIpInner() -> HtmlResult {
    #[cfg(feature = "csr")]
    {
        let res = use_future(|| async {
            gloo::net::http::Request::get("https://httpbin.org/ip")
                .send()
                .await?
                .text()
                .await
        })?;
        let result_html = match *res {
            Ok(ref res) => html! { res },
            Err(ref failure) => failure.to_string().into(),
        };
        return Ok(result_html);
    }

    #[cfg(not(feature = "csr"))]
    {
        Ok(html!(
            "this value was constructed during server-side rendering"
        ))
    }
}

#[function_component]
#[allow(non_snake_case)]
fn ClientIp() -> Html {
    let fallback = html!({ "loading..." });
    html!(
        <Suspense {fallback}>
            <ClientIpInner />
        </Suspense>
    )
}

#[function_component]
#[allow(non_snake_case)]
fn Counter() -> Html {
    let value = use_state(|| 0);
    let onclick = {
        let value = value.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            value.set(*value + 1);
        })
    };
    html!(
        <>
            {*value}
            <button onclick={onclick}>{"+1"}</button>
        </>
    )
}

fn switch(routes: Route) -> Html {
    html!(<div>
            <p>{"This is a demo of using Yew's server-side rendering with "}<a href="https://workers.cloudflare.com/">{"Cloudflare Workers"}</a>{". View the source code "}<a href="https://github.com/danya02/yew-ssr-wrangler-demo">{"on GitHub"}</a>{"."}</p>
            <p>{"You are currently viewing route: "}<code>{format!("{routes:?}")}</code>{" (try /, /page1, /page2, /page3, /404)"}</p>
            <p>{"When server was preparing this page, it had IP address: "}<ServerIp /></p>
            <p>{"When your browser is displaying this page, it has IP address: "}<ClientIp /></p>
            <p>{"Clicky counter: "}<Counter /></p>
        </div>
    )
}
