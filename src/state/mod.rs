use leptos::*;

mod map;

pub use map::MapState;

use crate::algorithm::Map;

#[allow(unused_braces)]
#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let map_state = create_rw_signal(MapState::new(Map::new()));

    provide_context(map_state);

    view! {
        {children()}
    }
}
