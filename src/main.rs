//! This project provides an interactive editor for metro maps

// Deny all default lints and warn on pedantic ones by default
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
// lots of casts have to be done back and forth between js and rust, even if f64 to i32 might in
// theory truncate.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
// wildcard imports are idiomatic with leptos.
#![allow(clippy::wildcard_imports)]
// having file and component function names be the same is idiomatic for leptos component files.
#![allow(clippy::module_name_repetitions)]

use leptos::*;

mod algorithm;
mod components;
mod models;
mod utils;

use components::{
    Home,
    StateProvider,
};

fn main() {
    // Initialize the logger with debug logging, so anything logged will be printed
    // to the console
    _ = console_log::init_with_level(log::Level::Debug);

    // Initialize the panic hook, which will print any panic that occurs to the
    // console
    console_error_panic_hook::set_once();

    // Start the application
    mount_to_body(|| view! { <App/> });
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
