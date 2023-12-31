fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    let renderer = yew::Renderer::<yew_app::App>::new();

    renderer.hydrate();
}
