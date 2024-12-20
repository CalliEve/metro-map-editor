//! Contains the [`Home`] page component

use leptos::prelude::*;

use crate::components::{
    molecules::ErrorBox,
    organisms::{
        CanvasControls,
        Navbar,
        Sidebar,
    },
};

/// The main page component.
#[component]
pub fn Home() -> impl IntoView {
    view! {
            <header>
                <Navbar/>
            </header>
            <ErrorBox/>
            <div class="grow flex flex-row justify-start">
                <div class="flex-none self-start self-stretch w-1/5 md:w-60">
                    <Sidebar/>
                </div>
                <div class="grow flex self-stretch">
                    <CanvasControls/>
                </div>
            </div>
    }
}
