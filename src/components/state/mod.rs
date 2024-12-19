//! Contains everything for keeping track of the current state of the page.

use leptos::*;

mod canvas;
mod error;
mod map;

pub use canvas::CanvasState;
pub use error::ErrorState;
pub use map::{
    ActionType,
    MapState,
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
    let error_state = create_rw_signal(error::ErrorState::new());

    provide_context(map_state);
    provide_context(error_state);

    view! {
        {children()}
    }
}
