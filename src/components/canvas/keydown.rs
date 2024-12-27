//! Contains the keydown event handler for the [`Canvas`] component.

use leptos::prelude::*;
use web_sys::KeyboardEvent;

use crate::{
    components::HistoryState,
    MapState,
};

/// Listener for the [keydown] event on the canvas.
///
/// [keydown]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
pub fn on_keydown(map_state_signal: &RwSignal<MapState>, ev: &KeyboardEvent) {
    if ev.key() == "Escape" {
        map_state_signal.update(|map_state| {
            map_state.clear_all_selections();
        });
    }

    if ev.key() == "z" && ev.ctrl_key() {
        map_state_signal.update(|map_state| {
            if let Some(map) = HistoryState::undo(
                map_state
                    .get_map()
                    .clone(),
            ) {
                map_state.set_map_no_history(map);
            }
        });
    }

    if ev.key() == "Z" && ev.ctrl_key() {
        map_state_signal.update(|map_state| {
            if let Some(map) = HistoryState::redo(
                map_state
                    .get_map()
                    .clone(),
            ) {
                map_state.set_map_no_history(map);
            }
        });
    }
}
