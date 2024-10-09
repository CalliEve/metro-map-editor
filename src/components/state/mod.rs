//! Contains everything for keeping track of the current state of the page.

use leptos::*;

mod canvas;
mod map;

pub use canvas::CanvasState;
pub use map::{
    MapState,
    RemoveType,
};

use crate::models::Map;

/// Provides all global state contexts to the page.
#[allow(unused_braces)]
#[component]
pub fn StateProvider(
    /// The contents of the page that will have access to the global state.
    children: Children,
) -> impl IntoView {
    let map_state = create_rw_signal(MapState::new(Map::new()));

    provide_context(map_state);

    view! {
        {children()}
    }
}
