//! Contains everything for keeping track of the current state of the page.

use leptos::*;

mod map;

pub use map::MapState;

use crate::models::Map;

/// Provides all global state contexts to the page.
#[allow(unused_braces)]
#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let map_state = create_rw_signal(MapState::new(Map::new()));

    provide_context(map_state);

    view! {
        {children()}
    }
}