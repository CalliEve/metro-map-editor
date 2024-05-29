use leptos::*;

mod map;

pub use map::MapState;

#[allow(unused_braces)]
#[component]
pub fn StateProvider(children: Children) -> impl IntoView {
    let map_state = create_rw_signal(MapState::default());

    provide_context(map_state);

    view! {
        {children()}
    }
}
