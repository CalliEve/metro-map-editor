//! This project provides an interactive editor for metro maps.
//! This file contains the main entry point for the application and starts the
//! editor on the web.

use leptos::*;
use metro_map_editor::*;

fn main() {
    if web_sys::window().is_some() {
        // Initialize the logger with debug logging, so anything logged will be printed
        // to the console
        _ = console_log::init_with_level(log::Level::Debug);

        // Initialize the panic hook, which will print any panic that occurs to the
        // console
        console_error_panic_hook::set_once();

        // Start the application
        mount_to_body(|| view! { <App/> });
    } else {
        // This is a worker; do nothing
    }
}

/// The App component that is the root for the application as a whole
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col h-screen max-w-screen">
            <StateProvider>
                <Home/>
            </StateProvider>
        </div>
    }
}
