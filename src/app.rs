use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/supermailer.css" />
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0.00);
    let on_click = move |_| set_count.update(|count| *count += 1.00);

    view! {
        <div class="bg-gray-800">
            <ProgressNav progress=count />
            <div class="text-white flex flex-col justify-center items-center min-h-screen">
                <h1>"Welcome to Leptos!"</h1>
                <button on:click=on_click>"Click Me: " {count}</button>
                <input
                    type="range"
                    max="100"
                    value=count
                    on:change=move |event| {
                        set_count(event_target_value(&event).parse::<f64>().unwrap());
                    }
                />
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
        <Title text="Leptos + Tailwindcss" />
        <main>
            <div class="font-mono text-white bg-gradient-to-tl from-blue-800 to-blue-500">
                <div class="flex">
                    <button
                        on:click=move |_| set_value.update(|value| *value += 1)
                        class="py-2 px-3 m-1 text-white bg-blue-700 rounded border-l-2 border-b-4 border-blue-800 shadow-lg"
                    >
                        "+"
                    </button>
                    <button class="py-2 px-3 m-1 text-white bg-blue-800 rounded border-l-2 border-b-4 border-blue-900 shadow-lg">
                        {value}
                    </button>
                    <button
                        on:click=move |_| set_value.update(|value| *value -= 1)
                        class="py-2 px-3 m-1 text-white bg-blue-700 rounded border-l-2 border-b-4 border-blue-800 shadow-lg"
                    >
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}

#[component]
fn ProgressNav(progress: ReadSignal<f64>) -> impl IntoView {
    let percentage = move || progress() / 100.0;
    view! {
        <div class="fixed top-0 right-0 left-0 h-0.5 bg-white">
            <div
                class="bg-gray-900 h-0.5 w-full origin-left transition-all"
                style:transform=move || format!("scaleX({})", percentage())
            />
        </div>
    }
}
