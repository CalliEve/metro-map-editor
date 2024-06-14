use leptos::*;

use crate::{
    algorithm::{Line, Map, Station},
    components::atoms::Button,
    state::MapState,
};

#[component]
pub fn Sidebar() -> impl IntoView {
    let map_state =
        use_context::<RwSignal<MapState>>().expect("to have found the global map state");

    view! {
        <div id="sidebar" class="h-full w-full flex flex-col bg-zinc-100 py-2 shadow-right shadow-dark-mild dark:shadow-black dark:bg-neutral-750 text-black dark:text-white px-2">
            <Button
                on_click=move |_| map_state.update(|state| state.set_map(testmap()))
                text="reset map" />
        </div>
    }
}

fn testmap() -> Map {
    let mut map = Map::new();

    map.add_line(Line::new(
        vec![
            Station::new((10, 10), None),
            Station::new((15, 15), None),
            Station::new((20, 25), None),
        ],
        "line 1",
    ));

    map.add_line(Line::new(
        vec![
            Station::new((20, 12), None),
            Station::new((25, 12), None),
            Station::new((30, 20), None),
        ],
        "line 2",
    ));

    map.add_line(Line::new(vec![Station::new((7, 5), None)], "line 3"));

    map
}
