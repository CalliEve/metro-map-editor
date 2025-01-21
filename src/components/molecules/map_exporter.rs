//! Contains the [`FileDownloader`] component.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    ImageEncodeOptions,
    OffscreenCanvas,
    Url,
};

use crate::{
    algorithms::redraw_canvas,
    components::{
        atoms::Button,
        MapState,
    },
};

/// A button that lets the user export and download the map as a png file.
#[component]
pub fn MapExporter() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let export_map = Action::new_local(move |()| {
        async move {
            let blob_promise = {
                let state = map_state.get_untracked();
                let (y_size, x_size) = state
                    .get_canvas_state()
                    .get_size();

                let canvas = OffscreenCanvas::new(x_size as u32, y_size as u32) // Full HD size (common resolution and not too big)
                    .expect("to create an offscreen canvas");

                redraw_canvas(&canvas, &state);

                let blob_options = ImageEncodeOptions::new();
                blob_options.set_type("image/png");

                canvas
                    .convert_to_blob_with_options(&blob_options)
                    .expect("to convert the canvas to a blob promise")
            };

            let blob = JsFuture::from(blob_promise)
                .await
                .expect("to await the promise")
                .dyn_into::<web_sys::Blob>()
                .expect("to convert the promise to a blob");

            let url = Url::create_object_url_with_blob(&blob)
                .expect("to create an object URL from the blob");

            let elem = document()
                .create_element("a")
                .expect("to create an anchor element")
                .dyn_into::<web_sys::HtmlAnchorElement>()
                .expect("to convert the element to an anchor element");

            elem.set_href(&url);
            elem.set_download("metro-map.png");
            elem.click();

            Url::revoke_object_url(&url).unwrap();
        }
    });

    view! {
        <Button text="To PNG" outlined=true can_focus=false on_click=Box::new(move |_| {export_map.dispatch(());})/>
    }
}
