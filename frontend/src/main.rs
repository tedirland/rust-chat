use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
        <div id="chat">
        <h1></h1>
        <p></p>

        </div>
        <div></div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
