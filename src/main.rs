use leptos::*;

mod components;

pub use components::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <header>
            <Navbar/>
        </header>
        <div class="h-full flex flex-row justify-start items-center">
            <div class="h-full flex-none w-1/5 md:w-52">
                <Sidebar/>
            </div>
            <div class="h-full grow">
                <Canvas/>
            </div>
        </div>
    }
}
