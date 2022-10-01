use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
      <>
        <h1>{ "Hello World" }</h1>
        <h2>{ "Subtitle" }</h2>
      </>
    }
}

fn main() {
    yew::start_app::<App>();
}