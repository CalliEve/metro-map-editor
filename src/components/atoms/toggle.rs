//! Contains the [`Toggle`] component.

use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

/// An input html element for boolean input displayed as a toggle button.
#[component]
pub fn Toggle<F, V>(
    /// The label on the input.
    text: &'static str,
    /// Gets called when the number input is changed.
    on_input: F,
    /// Gets called to set the current input value.
    #[prop(optional)]
    value: Option<V>,
) -> impl IntoView
where
    F: Fn(bool) + 'static,
    V: (Fn() -> bool) + Copy + 'static,
{
    let input_ref: NodeRef<html::Input> = create_node_ref();

    let id = text
        .to_lowercase()
        .replace(' ', "_");

    let parse_input = move |_| {
        let input = input_ref
            .get()
            .unwrap();
        let input_elem: &HtmlInputElement = input.unchecked_ref();

        on_input(input_elem.checked());
    };

    let input_class = "
        align-middle
        inline-block
        relative
        w-10
        h-5
        transition-all
        duration-200
        ease-in-out
        bg-gray-400
        rounded-full
        shadow-inner
        outline-none
        appearance-none
        cursor-pointer
        before:content-['']
        before:absolute
        before:w-5
        before:h-5
        before:rounded-[50%]
        before:top-0
        before:left-0
        before:scale-[1.1]
        before:shadow-[0_0.125rem_0.5rem_rgba(0,0,0,0.2)]
        before:bg-white
        before:duration-200
        before:ease-in-out
        checked:bg-indigo-400
        checked:before:left-5";

    view! {
    <div class="relative mb-3" data-twe-input-wrapper-init>
      <input
        type="checkbox"
        class=input_class
        id={id.clone()}
        _ref=input_ref
        on:input=parse_input
        checked=move || value.map_or(false, |v| v())
        prop:value=move || value.map(|v| v()) />
      <span
        class="pointer-events-none inline-block align-middle p-2 truncate pt-[0.37rem] text-neutral-500 peer-focus:text-primary dark:text-neutral-400 dark:peer-focus:text-primary"
        >{text}
      </span>
    </div>
        }
}
