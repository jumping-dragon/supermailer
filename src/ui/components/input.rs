use leptos::prelude::*;

#[component]
pub fn Input() -> impl IntoView {
    view! {
        <input
            class="flex py-2 px-3 w-full h-10 text-sm rounded-md border focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:cursor-not-allowed border-zinc-800 bg-zinc-950 ring-offset-zinc-950 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-white placeholder:text-muted-white focus-visible:ring-ring"
            placeholder="placeholder"
        />
    }
}
