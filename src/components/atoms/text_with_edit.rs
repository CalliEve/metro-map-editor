//! Contains the [`TextWithEdit`] component.

use ev::KeyboardEvent;
use leptos::*;

use crate::components::atoms::Button;

/// An html element for displaying text with an edit button and the ability to
/// edit that text.
#[component]
pub fn TextWithEdit<S, F>(
    /// The text to display.
    text: S,
    /// Gets called when the text is changed.
    on_edit: F,
    /// The label when hovering over the edit button.
    #[prop(optional)]
    edit_label: Option<S>,
) -> impl IntoView
where
    S: ToString + 'static,
    F: Fn(String) + Copy + 'static,
{
    let (editing, set_editing) = create_signal(false);
    let (text_input, set_text_input) = create_signal(text.to_string());

    // Generate the id for the input element and label.
    let id = edit_label
        .as_ref()
        .map_or_else(
            || format!("edit_{}", text.to_string()),
            |s| s.to_string(),
        )
        .to_lowercase()
        .replace(' ', "_");

    let on_click = move |_| {
        set_editing(true);
    };

    // Listeners for when the edit is being submitted.
    let on_done = move |_| {
        set_editing(false);
        on_edit(text_input.get());
    };
    let on_submit = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            set_editing(false);
            on_edit(text_input.get());
        }
    };

    // If the edit_label is not provided, use the text as the label.
    let edit_label = edit_label.map_or_else(
        || format!("Edit {}", text.to_string()),
        |s| s.to_string(),
    );
    // Clone to satisfy lifetimes and moves.
    let button_label = edit_label.clone();

    view! {
        <Show
            when=move || editing.get()
            fallback=move || view!{
                <span class="flex justify-between max-h-5">
                    {text_input} // FIXME: actually use the text from the input, this obfuscates the bug
                    <Button text={button_label.clone()} smaller=true on_click=Box::new(on_click)>
                        "edit"
                    </Button>
                </span>
            }>
            <div class="relative my-2 flex flex-row" data-twe-input-wrapper-init>
                <input
                    type="text"
                    maxlength="100"
                    class="grow peer block min-h-[auto] w-full rounded border-b-2 rounded-md border-solid border-blue-400 bg-transparent px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear peer-focus:text-primary motion-reduce:transition-none dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary dark:border-blue-600 focus:border-blue-600 dark:focus:border-blue-800"
                    id={id.clone()}
                    on:input=move |ev| set_text_input(event_target_value(&ev))
                    prop:value=move || text_input.get()
                    on:keydown=on_submit />
                <label
                    for={id.clone()}
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[1.6] text-neutral-500 peer-focus:text-primary -translate-y-[0.9rem] scale-[0.8] dark:text-neutral-400 dark:peer-focus:text-primary"
                    >{edit_label.clone()}
                </label>
                <Button text="finish editing" smaller=true on_click=Box::new(on_done.clone())>
                    "done"
                </Button>
            </div>
        </Show>
    }
}
