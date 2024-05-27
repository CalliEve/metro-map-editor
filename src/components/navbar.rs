use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
    <nav class="relative flex w-full items-center justify-between bg-zinc-100 py-2 shadow-dark-mild shadow-sm dark:shadow-neutral-900 dark:bg-neutral-750 lg:py-4">
      <div class="flex w-full items-center justify-between px-3">
        <div class="ms-2">
          <a class="text-2xl font-extrabold text-black dark:text-white" href="#">Metro Map Editor</a>
        </div>
      </div>
    </nav>
    }
}
