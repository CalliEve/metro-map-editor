//! Contains the [`Button`] component.

use leptos::{
    ev::MouseEvent,
    *,
};

/// A clickable button html element.
#[component]
pub fn Button<F>(
    /// The text displayed on the button.
    text: &'static str,
    /// Gets called when the button is clicked.
    on_click: F,
    /// If false the button is filled-in with one color, else just the border
    /// outline is shown.
    #[prop(optional)]
    outlined: bool,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let mut class = "inline-block rounded px-6 py-2 text-base font-semibold uppercase leading-normal shadow-primary-3 dark:shadow-neutral-950 hover:shadow-blue-900 dark:hover:shadow-neutral-900".to_owned();

    if outlined {
        class += " border-solid border-4 text-blue-400 border-blue-400 hover:text-blue-500 hover:border-blue-500 active:text-blue-600 active:border-blue-600 dark:text-blue-500 dark:border-blue-500 dark:hover:text-blue-600 dark:hover:border-blue-600 dark:active:text-blue-700 dark:active:border-blue-700";
    } else {
        class += " text-white bg-blue-400 hover:bg-blue-500 active:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-700 dark:active:bg-blue-800";
    }

    view! {
            <button
                type="button"
                class=class
                on:click=on_click>
                {text}
            </button>
    }
}
