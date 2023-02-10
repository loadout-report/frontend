#![feature(result_option_inspect)]

use tracing::info;
use tracing_subscriber::{
    fmt::{
        format::{FmtSpan, Pretty},
        time::UtcTime,
    },
    prelude::*,
};

mod app;
mod routes;
mod components;
mod client;

use app::App;

fn main() {let fmt_layer = tracing_subscriber::fmt::layer()
    .with_ansi(false)
    .with_timer(UtcTime::rfc_3339())
    .with_writer(tracing_web::MakeConsoleWriter)
    .with_span_events(FmtSpan::ACTIVE);
    let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
    info!("starting app");
    yew::Renderer::<App>::new().render();
}
