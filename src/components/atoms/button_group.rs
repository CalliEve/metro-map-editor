//! Contains the [`ButtonGroup`] component.

use leptos::*;

use super::button::ButtonProps;

/// A group of buttons
#[component]
pub fn ButtonGroup<S>(
    /// These will be transformed into [`super::Button`] elements.
    children: Vec<ButtonProps<S>>,
) -> impl IntoView
where
    S: ToString + 'static,
{
    let class = "max-w-full flex align-center gap-px [&>*]:flex-1 \
        [&>*:not(:first-child):not(:last-child)]:ml-0 \
        [&>*:not(:first-child):not(:last-child)]:rounded-none \
        [&>*:not(:only-child):first-child]:rounded-r-none \
        [&>*:not(:only-child):last-child]:rounded-l-none \
        [&>*:not(:only-child):last-child]:ml-0";

    view! {
        <div class=class>
            {children}
        </div>
    }
}
