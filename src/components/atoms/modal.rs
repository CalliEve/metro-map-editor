//! Contains the [`Modal`] component.

use leptos::{
    html::Div,
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

/// A generic modal that others can be based upon.
#[component]
pub fn Modal<C>(
    /// If the modal should be shown.
    show: ReadSignal<bool>,
    /// Gets called if the modal is closed by clicking outside the modal.
    on_close: C,
    /// The content of the modal.
    children: Children,
) -> impl IntoView
where
    C: Fn() + 'static,
{
    let modal_ref: NodeRef<Div> = NodeRef::new();

    let on_outside_click = move |e: MouseEvent| {
        // actual dom node that got clicked on
        let target_node = e
            .target()
            .and_then(|t| {
                t.dyn_ref::<web_sys::Node>()
                    .cloned()
            });

        // if the clicked node is outside the modal itself
        if !modal_ref
            .get()
            .unwrap()
            .contains(target_node.as_ref())
        {
            on_close();
        }
    };

    view! {
        <div
            id="modal"
            aria-hidden={move || if show() {"false"} else {"true"}}
            tabindex="-1"
            style:display=move || if show() {"flex"} else {"none"}
            class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"
            on:click=on_outside_click>
            <div node_ref=modal_ref class="relative p-4 w-full max-w-2xl max-h-full">
                // content
                <div class="relative bg-white rounded-lg shadow dark:bg-gray-700">
                    {children()}
                </div>
            </div>
        </div>
    }
}
