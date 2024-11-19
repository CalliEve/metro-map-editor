//! Contains the keydown event handler for the [`Canvas`] component.

use leptos::*;
use web_sys::KeyboardEvent;

use crate::MapState;

/// Listener for the [keydown] event on the canvas.
///
/// [keydown]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
pub fn on_keydown(map_state_signal: &RwSignal<MapState>, ev: &KeyboardEvent) {
    if ev.key() == "Escape" {
        map_state_signal.update(|map_state| {
            map_state.clear_all_selections();
        });
    }
}
