use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/supermailer.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="bg-green-100">
            <div class="flex flex-col items-center justify-center min-h-screen">
                <h1>"Welcome to Leptos!"</h1>
                <button on:click=on_click>"Click Me: " {count}</button>
                <Home />
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono">
                <div class="flex">
                    <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {value}
                    </button>
                    <button on:click=move |_| set_value.update(|value| *value -= 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}