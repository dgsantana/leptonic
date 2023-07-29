use leptos::*;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

mod app;

use crate::app::App;

fn main() {
    console_error_panic_hook::set_once();

    fmt()
        .with_writer(
            // To avoid trace events in the browser from showing their
            // JS backtrace, which is very annoying, in my opinion
            MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
        )
        // For some reason, if we don't do this in the browser, we get a runtime error.
        .without_time()
        .init();

    mount_to_body(|| {
        view! { <App/> }
    })
}
