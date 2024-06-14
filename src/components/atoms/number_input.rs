use std::i32;

use leptos::*;

#[component]
pub fn NumberInput<F, V>(
    text: &'static str,
    on_input: F,
    #[prop(optional)] value: Option<V>,
    #[prop(optional)] min: f64,
    #[prop(default = (i32::MAX) as f64)] max: f64,
) -> impl IntoView
where
    F: Fn(f64) + 'static,
    V: (Fn() -> f64) + Copy + 'static,
{
    let id = text.to_lowercase().replace(" ", "_");

    view! {
    <div class="relative mb-3" data-twe-input-wrapper-init>
      <input
        type="number"
        class="peer block min-h-[auto] w-full rounded border-b-2 rounded-md border-solid border-blue-400 bg-transparent px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear peer-focus:text-primary motion-reduce:transition-none dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary dark:border-blue-600 focus:border-blue-600 dark:focus:border-blue-800"
        id={id.clone()}
        on:input=move |ev| {on_input(event_target_value(&ev).parse().expect("number input does not give number"))}
        max=max
        min=min
        prop:value=move || min.max(value.map_or(0.0, |v| v())) />
      <label
        for={id}
        class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[1.6] text-neutral-500 peer-focus:text-primary -translate-y-[0.9rem] scale-[0.8] dark:text-neutral-400 dark:peer-focus:text-primary"
        >{text}
      </label>
    </div>
        }
}
