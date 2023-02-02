use yew::prelude::*;
use yew_hooks::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {

    html! {
        <div class="app">
            <header class="app-header">
                {"Loadout.report"}
            </header>
        </div>
    }
}
