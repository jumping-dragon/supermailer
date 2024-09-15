use leptos::*;

#[component]
pub fn Switch() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);
    let state = create_memo(move |_| if checked() { "checked" } else { "unchecked" });

    view! {
        <button
            data-state=state
            class="inline-flex items-center w-11 h-6 rounded-full border-2 border-transparent transition-colors cursor-pointer focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:cursor-not-allowed peer shrink-0 data-[state=checked]:bg-white data-[state=unchecked]:bg-zinc-800 focus-visible:ring-ring focus-visible:ring-offset-background"
            on:click=move |_| set_checked.set(!checked())
        >
            <span
                data-state=state
                class="block w-5 h-5 bg-black rounded-full ring-0 shadow-lg transition-transform pointer-events-none data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
            />
        </button>
    }
}
