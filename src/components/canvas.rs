use leptos::*;

#[component]
pub fn Canvas() -> impl IntoView {
    view! {
        <div class="h-full w-full flex bg-zinc-50 dark:bg-neutral-700 text-black dark:text-white">
            <canvas id="canvas" class="grow m-5"/>
        </div>
    }
}
