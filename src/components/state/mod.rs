//! Contains everything for keeping track of the current state of the page.

use leptos::prelude::*;

mod canvas;
mod error;
mod history;
mod interaction;
mod map;

pub use canvas::CanvasState;
pub use error::ErrorState;
pub use history::HistoryState;
pub use interaction::InteractionState;
pub use map::{
    ActionType,
    MapState,
};

use crate::models::Map;

/// Provides all global state contexts to the page.
#[component]
pub fn StateProvider(
    /// The contents of the page that will have access to the global state.
    children: Children,
) -> impl IntoView {
    let map_state = RwSignal::new(MapState::new(Map::new()));
    let error_state = RwSignal::new(error::ErrorState::new());
    let interaction_state = RwSignal::new(interaction::InteractionState::new());

    provide_context::<RwSignal<MapState>>(map_state);
    provide_context::<RwSignal<ErrorState>>(error_state);
    provide_context::<RwSignal<InteractionState>>(interaction_state);

    view! {
        <div class=move || format!("cursor-{}", interaction_state.get().get_cursor())>
        {children()}
        </div>
    }
}
