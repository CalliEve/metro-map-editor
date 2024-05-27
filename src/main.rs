use leptos::*;

mod algorithm;
mod components;

pub use components::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col h-screen max-w-screen">
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
        </div>
    }
}
