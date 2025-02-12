//! Contains the [`Button`] component.

use std::borrow::Borrow;

use leptos::{
    ev::MouseEvent,
    prelude::*,
};

use crate::components::state::InteractionState;

/// The type of the on click event handler.
type OnButtonClick = Box<dyn Fn(MouseEvent) + 'static>;

/// A clickable button html element.
#[component]
pub fn Button(
    /// The text displayed on the button.
    #[prop(into)]
    text: Signal<String>,
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
    /// If the button should be bigger, especially useful when having larger
    /// text or icons.
    #[prop(optional)]
    bigger: bool,
    /// If the button should be smaller, especially useful when having smaller
    /// text or icons or using the button in-line.
    #[prop(optional)]
    smaller: bool,
    /// If the button has been selected.
    #[prop(optional)]
    #[prop(into)]
    active: Signal<bool>,
    /// If focus can be held on the button.
    #[prop(optional)]
    can_focus: bool,
    /// Even if busy, the button should not be disabled.
    #[prop(optional)]
    never_too_busy: bool,
    /// If the button is disabled.
    #[prop(optional)]
    #[prop(into)]
    disabled: Signal<bool>,
    /// The children of the button, if any.
    /// If present, the button will show the text on hover.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    // let interaction_state = use_context::<RwSignal<InteractionState>>()
    //     .expect("to have found the global interaction state");

    let has_children = children.is_some();
    let is_disabled = move || {
        if never_too_busy {
            return disabled.get();
        }

        let interaction_state = use_context::<RwSignal<InteractionState>>()
            .expect("to have found the global interaction state");
        disabled.get()
            || interaction_state
                .get()
                .is_busy()
    };

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

    let class_func = move || {
        let mut class = "inline-block \
        text-center uppercase group relative \
        leading-snug shadow-neutral-800 \
        dark:shadow-neutral-950 hover:shadow-blue-900 \
        dark:hover:shadow-neutral-900"
            .to_owned();

        if overlay {
            class += " rounded-full text-xl font-bold";

            if bigger {
                class += " h-16 w-16";
            } else {
                class += " h-11 w-11";
            }
        } else {
            class += " rounded";

            if smaller {
                class += " text-xs font-semibold";
            } else if bigger {
                class += " text-xl font-semibold";
            } else {
                class += " text-sm font-semibold";
            }
        }

        if smaller {
            class += " px-1 py-0.5";
        } else {
            class += " px-4 py-1.5";
        }

        if outlined {
            class += &format!(
                " border-solid border-4 text-{color}-{base} \
                border-{color}-{base} \
                hover:text-{color}-{base_hover} \
                hover:border-{color}-{base_hover} \
                active:text-{color}-{base_active} \
                active:border-{color}-{base_active} \
                dark:text-{color}-{dark} \
                dark:border-{color}-{dark} \
                dark:hover:text-{color}-{dark_hover} \
                dark:hover:border-{color}-{dark_hover} \
                dark:active:text-{color}-{dark_active} \
                dark:active:border-{color}-{dark_active}"
            );

            if active.get() {
                class += &format!(
                    " text-{color}-{base_active} \
                border-{color}-{base_active} \
                dark:text-{color}-{dark_active} \
                dark:border-{color}-{dark_active}"
                );
            } else if can_focus {
                class += &format!(
                    " focus:text-{color}-{base_active} \
                focus:border-{color}-{base_active} \
                dark:focus:text-{color}-{dark_active} \
                dark:focus:border-{color}-{dark_active}"
                );
            }
        } else {
            class += &format!(
                " text-white bg-{color}-{base} \
                hover:bg-{color}-{base_hover} \
                active:bg-{color}-{base_active} \
                dark:bg-{color}-{dark} \
                dark:hover:bg-{color}-{dark_hover} \
                dark:active:bg-{color}-{dark_active}"
            );

            if active.get() {
                class += &format!(" bg-{color}-{base_active} dark:bg-{color}-{dark_active}");
            } else if can_focus {
                class +=
                    &format!(" focus:bg-{color}-{base_active} dark:focus:bg-{color}-{dark_active}");
            }
        }

        if is_disabled() {
            class += " cursor-not-allowed opacity-50";
        }

        class
    };

    let hover_class =
        "hidden group-hover:block rounded text-xs absolute z-10 width-fit p-1.5 bg-neutral-800 text-center bottom-[110%] right-0 whitespace-pre";

    let text_for_children = text;
    let children_signal = move || {
        children
            .clone()
            .map_or(
                AnyView::from(Fragment::from(
                    text_for_children
                        .borrow()
                        .get()
                        .into_any(),
                )),
                |c| c(),
            )
    };

    view! {
        <button
            type="button"
            class=class_func
            autofocus=active
            disabled=is_disabled
            on:click=on_click>
            <>
                {children_signal}
                <Show when=move || has_children>
                    <span class=hover_class>
                        {text}
                    </span>
                </Show>
            </>
        </button>
    }
}
