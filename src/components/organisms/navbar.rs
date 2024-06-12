use leptos::logging::log;
use leptos::*;

use crate::components::{atoms::Button, molecules::FileModal};

#[component]
pub fn Navbar() -> impl IntoView {
    let (show_file_modal, set_show_file_modal) = create_signal(false);

    view! {
    <nav id="navbar" class="pr-4 max-h-20 relative flex w-full items-center justify-between bg-zinc-100 py-2 shadow-dark-mild shadow-sm dark:shadow-neutral-900 dark:bg-neutral-750 lg:py-4">
      <div class="flex w-full items-center justify-between px-3">
        <div class="ms-2">
          <a class="text-2xl font-extrabold text-black dark:text-white" href="#">Metro Map Editor</a>
        </div>
        <Button text="Upload file" outlined=true on_click=move |_| set_show_file_modal(true)/>
      </div>
    </nav>
    <FileModal
        show=show_file_modal
        on_close=move || set_show_file_modal(false)
        on_submit=move |s| {set_show_file_modal(false); log!("{}", s)} />
    }
}
