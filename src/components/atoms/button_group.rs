//! Contains the [`ButtonGroup`] component.

use leptos::prelude::*;

use super::button::{
    Button,
    ButtonProps,
};

/// A group of buttons
#[component]
pub fn ButtonGroup(
    /// These will be transformed into [`super::Button`] elements.
    children: Vec<ButtonProps>,
) -> impl IntoView {
    let class = "max-w-full flex align-center gap-px \
        [&>*]:flex-1 [&>*]:max-w-[50%] \
        [&>*:not(:first-child):not(:last-child)]:ml-0 \
        [&>*:not(:first-child):not(:last-child)]:rounded-none \
        [&>*:not(:only-child):first-child]:rounded-r-none \
        [&>*:not(:only-child):last-child]:rounded-l-none \
        [&>*:not(:only-child):last-child]:ml-0";

    let children = AnyView::from(
        children
            .into_iter()
            .map(|c| Button(c).into_any())
            .collect::<Fragment>(),
    );

    view! {
        <div class=class>
            {children}
        </div>
    }
}
