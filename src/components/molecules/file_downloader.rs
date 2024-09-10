//! Contains the [`FileDownloader`] component.

use leptos::*;
use wasm_bindgen::{
    JsCast,
    JsValue,
};
use web_sys::{
    js_sys::Array,
    Blob,
    BlobPropertyBag,
    Url,
};

use super::FileType;
use crate::{
    components::{
        atoms::Button,
        MapState,
    },
    unwrap_or_return,
    utils::json::encode_map,
};

/// A modal that lets the user download a file representing the map.
#[component]
pub fn FileDownloader() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let download_map = move |file_type: FileType| {
        let encoded = unwrap_or_return!(match file_type {
            FileType::Json => {
                let state = map_state.get_untracked();
                encode_map(
                    state.get_map(),
                    state.get_canvas_state(),
                )
            },
            FileType::GraphML => return,
        });
        let options = BlobPropertyBag::new();
        options.set_type(file_type.to_mime_type());

        let str_sequence = std::iter::once(JsValue::from_str(&encoded)).collect::<Array>();
        let blob = unwrap_or_return!(Blob::new_with_str_sequence_and_options(
            &str_sequence,
            &options
        ));
        let url = unwrap_or_return!(Url::create_object_url_with_blob(&blob));

        let elem = unwrap_or_return!(document().create_element("a"))
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .expect("to convert the element to an anchor element");

        elem.set_href(&url);
        elem.set_download(&format!(
            "metro-map.{}",
            match file_type {
                FileType::Json => "json",
                FileType::GraphML => "graphml",
            }
        ));
        elem.click();

        unwrap_or_return!(Url::revoke_object_url(&url));
    };

    view! {
        <Button text="Download Map" outlined=true can_focus=false on_click=Box::new(move |_| download_map(FileType::Json))/>
    }
}
