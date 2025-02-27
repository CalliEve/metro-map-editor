//! Contains the [`Navbar`] component.

use leptos::prelude::*;

use crate::{
    components::{
        atoms::{
            Button,
            Link,
            LinkIcon,
        },
        molecules::{
            FileDownloader,
            FileModal,
            FileType,
            MapExporter,
            SettingsModal,
        },
        ErrorState,
        MapState,
    },
    unwrap_or_return,
    utils::{
        graphml,
        json,
    },
};

/// The navbar at the top of the page.
/// Also contains the modal for uploading a file.
#[component]
pub fn Navbar() -> impl IntoView {
    let (show_file_modal, set_show_file_modal) = signal(false);
    let (show_settings_modal, set_show_settings_modal) = signal(false);
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");
    let error_state =
        use_context::<RwSignal<ErrorState>>().expect("to have found the global error state");

    let on_submit = move |file_type: FileType, s: String| {
        set_show_file_modal(false);

        map_state.update(|state| {
            let map = unwrap_or_return!(
                error_state,
                match file_type {
                    FileType::Json => {
                        json::decode_map(&s, state.get_canvas_state())
                    },
                    FileType::GraphML => {
                        graphml::decode_map(&s, state.get_canvas_state())
                    },
                }
            );

            state.set_map(map.clone());
            state.set_last_loaded(map);
        });
    };

    view! {
    <nav id="navbar" class="pr-4 max-h-20 relative flex w-full items-center justify-between bg-zinc-100 py-2 shadow-dark-mild shadow-sm dark:shadow-neutral-900 dark:bg-neutral-750 lg:py-4">
      <div class="flex w-full items-center justify-between px-3">
        <div class="ms-2">
          <a class="text-2xl font-extrabold text-black dark:text-white" href="#">Metro Map Editor</a>
        </div>
        <div class="flex flex-row items-center space-x-3" >
            <LinkIcon link=Link::Docs />
            <LinkIcon link=Link::GitHub />
            <Button text="Advanced Settings" outlined=true can_focus=true on_click=Box::new(move |_| set_show_settings_modal(true))/>
            <MapExporter/>
            <FileDownloader/>
            <Button text="Upload File" outlined=true can_focus=true on_click=Box::new(move |_| set_show_file_modal(true))/>
        </div>
      </div>
    </nav>
    <FileModal
        show=show_file_modal
        on_close=move || set_show_file_modal(false)
        on_submit=on_submit />
    <SettingsModal
        show=show_settings_modal
        on_close=move || set_show_settings_modal(false) />
    }
}
