//! Contains the [`Button`] component.

use leptos::{
    ev::MouseEvent,
    *,
};

type OnButtonClick = Box<dyn Fn(MouseEvent) + 'static>;

/// A clickable button html element.
#[component]
pub fn Button(
    /// The text displayed on the button.
    text: &'static str,
    /// Gets called when the button is clicked.
    on_click: OnButtonClick,
    /// If false the button is filled-in with one color, else just the border
    /// outline is shown.
    #[prop(optional)]
    outlined: bool,
    /// If the button should be colored red (is blue otherwise).
    #[prop(optional)]
    danger: bool,
    /// If the button is an overlay button.
    #[prop(optional)]
    overlay: bool,
    /// If the button has been selected.
    #[prop(optional)]
    #[prop(into)]
    active: Signal<bool>,
) -> impl IntoView {
    let color = if danger {
        "red"
    } else if overlay {
        "gray"
    } else {
        "blue"
    };

    let base = if danger { 600 } else { 400 };
    let base_hover = base + 100;
    let base_active = base + 200;
    let dark = base + 200;
    let dark_hover = dark + 100;
    let dark_active = if dark >= 800 { 950 } else { dark + 200 };

    let mut class = "inline-block px-4 \
        py-1.5 text-center uppercase \
        leading-snug shadow-neutral-800 \
        dark:shadow-neutral-950 hover:shadow-blue-900 \
        dark:hover:shadow-neutral-900"
        .to_owned();

    if overlay {
        class += " rounded-full text-xl font-bold h-11 w-11";
    } else {
        class += " rounded text-sm font-semibold";
    }

    if outlined {
        class += &format!(
            " border-solid border-4 text-{color}-{base} \
            border-{color}-{base} hover:text-{color}-{base_hover} \
            hover:border-{color}-{base_hover} \
            active:text-{color}-{base_active} \
            active:border-{color}-{base_active} \
            focus:text-{color}-{base_active} \
            focus:border-{color}-{base_active} \
            dark:text-{color}-{dark} \
            dark:border-{color}-{dark} \
            dark:hover:text-{color}-{dark_hover} \
            dark:hover:border-{color}-{dark_hover} \
            dark:active:text-{color}-{dark_active} \
            dark:active:border-{color}-{dark_active} \
            dark:focus:text-{color}-{dark_active} \
            dark:focus:border-{color}-{dark_active}"
        );
    } else {
        class += &format!(
            " text-white bg-{color}-{base} \
            hover:bg-{color}-{base_hover} \
            active:bg-{color}-{base_active} \
            focus:bg-{color}-{base_active} \
            dark:bg-{color}-{dark} \
            dark:hover:bg-{color}-{dark_hover} \
            dark:active:bg-{color}-{dark_active} \
            dark:focus:bg-{color}-{dark_active}"
        );
    }

    class = class.replace("1000", "950");

    view! {
        <button
            type="button"
            class=class
            focus=active
            on:click=on_click>
            {text}
        </button>
    }
}
