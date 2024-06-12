use leptos::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::HtmlInputElement;

use crate::components::atoms::Button;

fn get_file<S>(input: &HtmlInputElement, on_submit: S)
where
    S: Fn(String) + 'static,
{
    let cb = Closure::new(move |v: JsValue| {
        on_submit(v.as_string().expect("file contents should be a string"));
    });

    input
        .files()
        .and_then(|l| l.item(0))
        .map(|f| f.text())
        .iter()
        .for_each(|p| {
            let _ = p.then(&cb);
        });

    cb.forget();
}

#[component]
pub fn FileModal<S, C>(show: ReadSignal<bool>, on_submit: S, on_close: C) -> impl IntoView
where
    S: Fn(String) + 'static + Copy,
    C: Fn() + 'static,
{
    let modal_ref: NodeRef<html::Div> = create_node_ref();
    let input_ref: NodeRef<html::Input> = create_node_ref();

    view! {
        <div
            id="file-modal"
            aria-hidden={move || if show() {"false"} else {"true"}}
            tabindex="-1"
            style:display=move || if show() {"flex"} else {"none"}
            class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"
            on:click=move |e| if !modal_ref.get().unwrap().contains(e.target().and_then(|t| t.dyn_ref::<web_sys::Node>().cloned()).as_ref()) {on_close()}>
            <div _ref=modal_ref class="relative p-4 w-full max-w-2xl max-h-full">
                // content
                <div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
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
                        <Button text="Upload File" on_click=move |_| get_file(&input_ref.get().unwrap(), on_submit)/>
                    </div>
                </div>
            </div>
        </div>
    }
}
