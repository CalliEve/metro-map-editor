//! Contains the [`Navbar`] component.

use leptos::*;

use crate::{
    components::{
        atoms::Button,
        molecules::FileModal,
        MapState,
    },
    utils::decode_map,
};

/// The navbar at the top of the page.
/// Also contains the modal for uploading a file.
#[component]
pub fn Navbar() -> impl IntoView {
    let (show_file_modal, set_show_file_modal) = create_signal(false);
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let on_submit = move |s: String| {
        set_show_file_modal(false);
        map_state.update(|state| state.set_map(decode_map(&s, state.get_canvas_state()).unwrap()));
    };

    view! {
    <nav id="navbar" class="pr-4 max-h-20 relative flex w-full items-center justify-between bg-zinc-100 py-2 shadow-dark-mild shadow-sm dark:shadow-neutral-900 dark:bg-neutral-750 lg:py-4">
      <div class="flex w-full items-center justify-between px-3">
        <div class="ms-2">
          <a class="text-2xl font-extrabold text-black dark:text-white" href="#">Metro Map Editor</a>
        </div>
        <Button text="Upload file" outlined=true on_click=Box::new(move |_| set_show_file_modal(true))/>
      </div>
    </nav>
    <FileModal
        show=show_file_modal
        on_close=move || set_show_file_modal(false)
        on_submit=on_submit />
    }
}
