use leptos::*;

mod algorithm;
mod components;
mod state;

use components::Page;
use state::StateProvider;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col h-screen max-w-screen">
            <StateProvider>
                <Page/>
            </StateProvider>
        </div>
    }
}
