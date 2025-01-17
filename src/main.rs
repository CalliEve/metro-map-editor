//! This project provides an interactive editor for metro maps.
//! This file contains the main entry point for the application and starts the
//! editor on the web.

use leptos::prelude::*;
use metro_map_editor::*;

fn main() {
    #[cfg(feature = "heatmap")]
    if cfg!(feature = "heatmap") {
        // Run the heatmap algorithm
        utils::heatmap_data::run_heatmap();
        return;
    }

    if web_sys::window().is_some() {
        // Initialize the panic hook, which will print any panic that occurs to the
        // console
        console_error_panic_hook::set_once();

        // Start the application
        mount_to_body(App);
    } else {
        // This is a worker; do nothing
    }
}

/// The App component that is the root for the application as a whole
#[component]
fn App() -> impl IntoView {
    view! {
        <StateProvider>
            <div class="flex flex-col h-screen max-w-screen">
                <Home/>
            </div>
        </StateProvider>
    }
}
