use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Sequence inspector" }</h1>
            <p>{ "Hello from Yew and WebAssembly!" }</p>
            <p>{ "Edit frontend/src/main.rs and save — trunk reloads the page for you." }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
