use leptos::*;

#[component]
pub fn Badge(children: Children) -> impl IntoView {
    view! {
        <div class="inline-flex items-center py-0.5 px-2.5 text-xs font-semibold text-black bg-white rounded-full border transition-colors hover:bg-gray-300 focus:ring-2 focus:ring-offset-2 focus:outline-none focus:ring-ring">
            {children()}
        </div>
    }
}
