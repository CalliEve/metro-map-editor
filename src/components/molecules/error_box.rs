//! Contains the [`ErrorBox`] component.

use leptos::*;
use wasm_bindgen::{
    prelude::Closure,
    JsCast,
};

use crate::components::ErrorState;

/// A pop-up box for displaying errors.
#[component]
pub fn ErrorBox() -> impl IntoView {
    let error_state =
        use_context::<RwSignal<ErrorState>>().expect("to have found the global error state");

    let on_click = move |_| {
        error_state.update(|state| state.clear_error());
    };

    let has_error = move || {
        error_state
            .get()
            .has_error()
    };
    let error_message = move || {
        let err = error_state
            .get()
            .get_error()
            .map(|e| e.to_user_friendly_string());

        if err.is_some() {
            let f = Closure::wrap(Box::new(move || {
                error_state.update(|state| state.clear_error());
            }) as Box<dyn Fn()>);
            window()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    f.as_ref()
                        .unchecked_ref(),
                    3000,
                )
                .unwrap();
            f.forget();
        }

        err
    };

    view! {
        <Show when=has_error>
            <div
            id="error-box"
            tabindex="-1"
            style:pointer-events="none"
            class="overflow-y-auto overflow-x-hidden fixed flex top-0 right-0 left-0 z-50 justify-center items-start w-full md:inset-0 h-[calc(100%-1rem)] max-h-full">
                <div class="mt-3.5 z-50 w-fit min-w-14" on:click=on_click>
                    <div class="bg-red-500 text-white top-2 z-50 p-3.5 rounded-lg text-lg cursor-pointer relative">
                        <span class="absolute -top-1 right-2 text-base">x</span>
                        <span>{error_message}</span>
                    </div>
                </div>
            </div>
        </Show>
    }
}
