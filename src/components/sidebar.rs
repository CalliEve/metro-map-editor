use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div id="sidebar" class="h-full w-full flex flex-col bg-zinc-100 py-2 shadow-right shadow-dark-mild dark:shadow-black dark:bg-neutral-750 text-black dark:text-white px-2">
            <div class="px-3 py-3 w-full">sidebar</div>
        </div>
    }
}
