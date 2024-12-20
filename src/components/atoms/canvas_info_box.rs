//! Contains the [`CanvasInfoBox`] component.

use leptos::{
    html::Div,
    prelude::*,
};

use crate::MapState;

/// A generic canvas info box that others can be based upon.
#[allow(clippy::needless_pass_by_value)] // cannot be a reference because of the `Fn` trait
#[component]
pub fn CanvasInfoBox<S>(
    /// The title of the info box,
    title: S,
    /// If the info box should be shown.
    click_position: Signal<Option<(f64, f64)>>,
    /// The body of the info box if applicable.
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView
where
    S: ToString + 'static,
{
    let info_box_ref: NodeRef<Div> = NodeRef::new();
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    let show = move || {
        click_position
            .get()
            .is_some()
    };
    let has_children = children.is_some();

    let left = move || {
        let map_pos = click_position
            .get()
            .map_or(0.0, |(x, _)| x);
        let sidebar_width = map_state
            .get()
            .get_canvas_state()
            .get_neighbor_sizes()
            .1;

        // Offset by 15px to the left, aka 1rem - 1px
        let screen_pos = map_pos + sidebar_width + 2.0;
        format!("{screen_pos}px")
    };
    let top = move || {
        let map_pos = click_position
            .get()
            .map_or(0.0, |(_, y)| y);
        let navbar_height = map_state
            .get()
            .get_canvas_state()
            .get_neighbor_sizes()
            .0;

        // Offset by 15px to the top, aka 1rem - 1px
        let screen_pos = map_pos + navbar_height + 2.0;
        format!("{screen_pos}px")
    };

    view! {
        <div
            id="canvas-info-box"
            aria-hidden={move || if show() {"false"} else {"true"}}
            tabindex="-1"
            style:display=move || if show() {"block"} else {"none"}
            class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 w-full md:inset-0 h-[calc(100%-0.125rem)] max-h-full"
            style:pointer-events="none">
            <div
                node_ref=info_box_ref
                style:pointer-events="auto"
                style:top=top
                style:left=left
                class="absolute w-80 max-w-2xl max-h-full">
                // title
                <div class=move || String::from("text-lg px-2 pb-0.5 font-semibold bg-white shadow dark:bg-gray-700") + if has_children { " rounded-t-lg" } else {" rounded-lg"}>
                    <h2>{title.to_string()}</h2>
                </div>
                <div
                    aria-hidden={move || if has_children {"false"} else {"true"}}
                    style:display=move || if has_children {"block"} else {"none"}
                    class="relative leading-tight px-2 pb-1 rounded-b-lg whitespace-pre-line bg-white shadow dark:bg-gray-700">
                    <hr/>
                    // content
                    <div>
                        {children.map_or(view!{<div></div>}.into_any(), |c| c())}
                    </div>
                </div>
            </div>
        </div>
    }
}
