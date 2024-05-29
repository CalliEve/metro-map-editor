use leptos::*;

use super::{canvas::Canvas, navbar::Navbar, sidebar::Sidebar};

#[component]
pub fn Page() -> impl IntoView {
    view! {
            <header>
                <Navbar/>
            </header>
            <div class="grow flex flex-row justify-start">
                <div class="flex-none self-start self-stretch w-1/5 md:w-52">
                    <Sidebar/>
                </div>
                <div class="grow flex self-stretch">
                    <Canvas/>
                </div>
            </div>
    }
}
