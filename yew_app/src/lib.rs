use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

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

fn switch(routes: Route) -> Html {
    html!(<p>{"You are currently viewing route:"}{format!("{routes:?}")}</p>)
}
