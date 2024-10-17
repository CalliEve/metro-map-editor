//! Contains the [`Button`] component.

use leptos::{
    ev::MouseEvent,
    *,
};
use leptos_dom::Text;

/// The type of the on click event handler.
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
    /// If the button should be bigger, especially useful when having larger
    /// text or icons.
    #[prop(optional)]
    bigger: bool,
    /// If the button has been selected.
    #[prop(optional)]
    #[prop(into)]
    active: Signal<bool>,
    /// If focus can be held on the button.
    #[prop(optional)]
    can_focus: bool,
    /// The children of the button, if any.
    /// If present, the button will show the text on hover.
    #[prop(optional)]
    children: Option<Children>,
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

    let class_func = move || {
        let mut class = "inline-block group px-4 \
        py-1.5 text-center uppercase \
        leading-snug shadow-neutral-800 \
        dark:shadow-neutral-950 hover:shadow-blue-900 \
        dark:hover:shadow-neutral-900"
            .to_owned();

        if overlay && bigger {
            class += " rounded-full text-xl font-bold h-16 w-16";
        } else if overlay {
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
        class
    };

    let has_children = children.is_some();

    let mut hover_class = "hidden group-hover:block text-xs absolute z-10 ".to_owned();
    if bigger {
        hover_class += "-top-8 -left-2.5";
    } else {
        hover_class += "-top-7 left-0.5";
    }

    view! {
        <button
            type="button"
            class=class_func
            focus=active
            on:click=on_click>
            <>
                {children.map_or(Fragment::from(Text::new(text.into()).into_view()), |c| c())}
                <div class=hover_class>
                    {has_children.then_some(text)}
                </div>
            </>
        </button>
    }
}
