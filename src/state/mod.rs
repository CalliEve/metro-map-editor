use leptos::*;

use crate::algorithm::Map;

#[derive(Clone, Debug, Default)]
pub struct MapState {
    pub map: Option<Map>,
}

impl MapState {
    pub fn new(map: Map) -> Self {
        Self { map: Some(map) }
    }
}

#[allow(unused_braces)]
#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let map_state = create_rw_signal(MapState::default());

    provide_context(map_state);

    view! {
        {children()}
    }
}
