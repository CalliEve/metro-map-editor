//! Contains the [`FileModal`] component.

use std::path::Path;

use ev::MouseEvent;
use leptos::*;
use wasm_bindgen::{
    closure::Closure,
    JsCast,
    JsValue,
};
use web_sys::HtmlInputElement;

use crate::{
    components::atoms::{
        Button,
        Modal,
    },
    unwrap_or_return,
    Error,
};

/// The accepted file types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// JSON file type.
    Json,
    /// GraphML file type.
    GraphML,
}

impl FileType {
    /// Returns the MIME type of the file type.
    pub fn to_mime_type(self) -> &'static str {
        match self {
            FileType::Json => "application/json",
            FileType::GraphML => "application/graphml+xml",
        }
    }
}

/// Gets the file uploaded to the input element by the user and passes its
/// contents to the provided `on_submit` callback function.
fn get_file<S>(input: &HtmlInputElement, on_submit: S)
where
    S: Fn(FileType, String) + 'static,
{
    let Some(file) = input
        .files()
        .and_then(|l| l.item(0))
    else {
        return;
    };

    let file_name = file.name();
    let file_ext = Path::new(&file_name).extension();

    let file_type = if file_ext.map_or(false, |ext| {
        ext.eq_ignore_ascii_case("json")
    }) {
        FileType::Json
    } else if file_ext.map_or(false, |ext| {
        ext.eq_ignore_ascii_case("graphml")
    }) {
        FileType::GraphML
    } else {
        return;
    };

    let cb = Closure::new(move |v: JsValue| {
        on_submit(
            file_type,
            unwrap_or_return!(v
                .as_string()
                .ok_or(Error::other(
                    "file contents should be a string"
                ))),
        );
    });

    let _ = file
        .text()
        .then(&cb);

    cb.forget();
}

/// A modal that asks the user to upload a file.
#[component]
pub fn FileModal<S, C>(
    /// If the modal should be shown.
    show: ReadSignal<bool>,
    /// Gets called on file submit with the contents of the file.
    on_submit: S,
    /// Gets called if the modal is closed without submit (the user clicks
    /// outside of the modal).
    on_close: C,
) -> impl IntoView
where
    S: Fn(FileType, String) + 'static + Copy,
    C: Fn() + 'static,
{
    let input_ref: NodeRef<html::Input> = create_node_ref();

    view! {
        <Modal show=show on_close=on_close>
            // body
            <div class="p-4 md:p-5 space-y-4">
                <label
                    for="file-form"
                    class="mb-2 inline-block text-neutral-500 dark:text-neutral-400">
                    "input file to upload to the map editor"
                </label>
                <input
                    id="file-form"
                    _ref=input_ref
                    type="file"
                    accept=".json, .graphml"
                    class="relative m-0 block w-full min-w-0 flex-auto cursor-pointer rounded border border-solid border-secondary-500 bg-transparent bg-clip-padding px-3 py-[0.32rem] text-base font-normal text-surface transition duration-300 ease-in-out file:-mx-3 file:-my-[0.32rem] file:me-3 file:cursor-pointer file:overflow-hidden file:rounded-none file:border-0 file:border-e file:border-solid file:border-inherit file:bg-transparent file:px-3  file:py-[0.32rem] file:text-surface focus:border-primary focus:text-gray-700 focus:shadow-inset focus:outline-none dark:border-white/70 dark:text-white  file:dark:text-white"/>
            </div>
            // footer
            <div class="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600">
                <Button text="Upload File" on_click=Box::new(move |_| get_file(&input_ref.get().unwrap(), on_submit))/>
            </div>
        </Modal>
    }
}
