//! Contains the [`NumberInput`] component.

use leptos::prelude::*;

/// An input html element for number input.
#[component]
pub fn NumberInput<F, V>(
    /// The label on the input.
    text: &'static str,
    /// Gets called when the number input is changed.
    on_input: F,
    /// Gets called to set the current input value.
    #[prop(optional)]
    value: Option<V>,
    /// The minimum value (default: 0)
    #[prop(optional)]
    min: f64,
    /// The maximum value (default: i32::MAX)
    #[prop(default = (i32::MAX) as f64)]
    max: f64,
    /// The step value (default: 1.0)
    #[prop(default = 1.0)]
    step: f64,
) -> impl IntoView
where
    F: Fn(f64) + 'static,
    V: (Fn() -> f64) + Copy + Send + 'static,
{
    let id = text
        .to_lowercase()
        .replace(' ', "_");

    let parse_input = move |ev| {
        let val = event_target_value(&ev);

        on_input(
            val.parse()
                .unwrap_or(min),
        );
    };

    view! {
    <div class="relative mb-3" data-twe-input-wrapper-init>
      <input
        type="number"
        class="peer block min-h-[auto] w-full rounded border-b-2 rounded-md border-solid border-blue-400 bg-transparent px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear peer-focus:text-primary motion-reduce:transition-none dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary dark:border-blue-600 focus:border-blue-600 dark:focus:border-blue-800"
        id={id.clone()}
        on:input=parse_input
        max=max
        min=min
        step=step
        prop:value=move || value.map(|v| v().max(min)) />
      <label
        for={id}
        class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[1.6] text-neutral-500 peer-focus:text-primary -translate-y-[0.9rem] scale-[0.8] dark:text-neutral-400 dark:peer-focus:text-primary"
        >{text}
      </label>
    </div>
        }
}
