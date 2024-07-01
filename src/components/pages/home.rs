//! Contains the [`Home`] page component

use leptos::*;

use crate::components::organisms::{
    CanvasControls,
    Navbar,
    Sidebar,
};

/// The main page component.
#[component]
pub fn Home() -> impl IntoView {
    view! {
            <header>
                <Navbar/>
            </header>
            <div class="grow flex flex-row justify-start">
                <div class="flex-none self-start self-stretch w-1/5 md:w-52">
                    <Sidebar/>
                </div>
                <div class="grow flex self-stretch">
                    <CanvasControls/>
                </div>
            </div>
    }
}
