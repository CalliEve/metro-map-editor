use leptos::*;

use crate::{
    algorithm::{Line, Map, Station},
    state::MapState,
};

#[component]
pub fn Sidebar() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    view! {
        <div id="sidebar" class="h-full w-full flex flex-col bg-zinc-100 py-2 shadow-right shadow-dark-mild dark:shadow-black dark:bg-neutral-750 text-black dark:text-white px-2">
            <button
                type="button"
                class="inline-block rounded bg-blue-400 px-6 pb-2 pt-2 text-md font-medium uppercase leading-normal text-white shadow-primary-3 hover:bg-blue-500 hover:shadow-blue-900 active:bg-blue-600 dark:bg-blue-600 dark:shadow-neutral-950 dark:hover:bg-blue-700 dark:hover:shadow-neutral-900 dark:active:bg-blue-800"
                on:click=move |_| map_state.set(MapState::new(testmap()))>
                "reset map"
            </button>
        </div>
    }
}

fn testmap() -> Map {
    let mut map = Map::new();

    map.add_line(Line::new(vec![
        Station::new((10, 10)),
        Station::new((15, 15)),
        Station::new((20, 25)),
    ]));

    map.add_line(Line::new(vec![
        Station::new((20, 12)),
        Station::new((25, 12)),
        Station::new((30, 20)),
    ]));

    map
}
