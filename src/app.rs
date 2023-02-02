use log::*;
use obfstr::obfstr;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;
use crate::components::client::ClientProvider;
use crate::components::nav::Nav;
use crate::routes::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    let client = crate::client::Client::with_api_key(obfstr!(env!("D2_API_KEY")));

    html! {
        <ClientProvider client={client}>
            <BrowserRouter>
                <Nav />
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </ClientProvider>
    }
}
