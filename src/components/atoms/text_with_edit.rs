//! Contains the [`TextWithEdit`] component.

use leptos::{
    prelude::*,
    text_prop::TextProp,
};
use web_sys::KeyboardEvent;

use crate::components::atoms::Button;

/// An html element for displaying text with an edit button and the ability to
/// edit that text.
#[component]
pub fn TextWithEdit<F>(
    /// The text to display.
    #[prop(into)]
    text: TextProp,
    /// Gets called when the text is changed.
    on_edit: F,
    /// The label when hovering over the edit button.
    #[prop(optional)]
    #[prop(into)]
    edit_label: Option<TextProp>,
) -> impl IntoView
where
    F: Fn(String) + Copy + Send + Sync + 'static,
{
    let (editing, set_editing) = signal(false);
    let (text_input, set_text_input) = signal(String::new());

    // Generate the id for the input element and label.
    let edit_label_for_id = edit_label.clone();
    let text_for_id = text.clone();
    let id = Signal::derive(move || {
        edit_label_for_id
            .clone()
            .map_or_else(
                || format!("edit_{}", text_for_id.get()),
                |s| {
                    s.get()
                        .to_string()
                },
            )
            .to_lowercase()
            .replace(' ', "_")
    });

    let on_click = move |_| {
        set_editing(true);
    };

    // Listeners for when the edit is being submitted.
    let on_done = move |_| {
        set_editing(false);
        on_edit(text_input.get());
        set_text_input(String::new());
    };
    let on_submit = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            set_editing(false);
            on_edit(text_input.get());
            set_text_input(String::new());
        }
    };

    // If the edit_label is not provided, use the text as the label.
    let text_for_edit_label = text.clone();
    let edit_label = Signal::derive(move || {
        edit_label
            .as_ref()
            .map_or_else(
                || format!("Edit {}", text_for_edit_label.get()),
                |s| {
                    s.get()
                        .to_string()
                },
            )
    });
    // Clone to satisfy lifetimes and moves.
    let button_label = edit_label;

    let text_for_effect = text.clone();
    Effect::new(move |_| {
        if text_input
            .get()
            .is_empty()
        {
            set_text_input(
                text_for_effect
                    .get()
                    .to_string(),
            );
        }
    });

    view! {
        <Show
            when=move || editing.get()
            fallback=move || view!{
                <span class="flex justify-between max-h-5">
                    {text.get()}
                    <Button text={button_label} smaller=true on_click=Box::new(on_click) never_too_busy=true>
                        "edit"
                    </Button>
                </span>
            }>
            <div class="relative my-2 flex flex-row" data-twe-input-wrapper-init>
                <input
                    type="text"
                    maxlength="100"
                    class="grow peer block min-h-[auto] w-full rounded border-b-2 rounded-md border-solid border-blue-400 bg-transparent px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear peer-focus:text-primary motion-reduce:transition-none dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary dark:border-blue-600 focus:border-blue-600 dark:focus:border-blue-800"
                    id={id}
                    on:input=move |ev| set_text_input(event_target_value(&ev))
                    prop:value=move || text_input.get()
                    on:keydown=on_submit />
                <label
                    for={id}
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[1.6] text-neutral-500 peer-focus:text-primary -translate-y-[0.9rem] scale-[0.8] dark:text-neutral-400 dark:peer-focus:text-primary"
                    >{edit_label}
                </label>
                <Button text="finish editing" smaller=true on_click=Box::new(on_done) never_too_busy=true>
                    "done"
                </Button>
            </div>
        </Show>
    }
}
