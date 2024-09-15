use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod home;
use crate::ui::home::HomePage;
pub mod mail;
use crate::ui::mail::MailPage;
pub mod components;

#[component]
pub fn Ui() -> impl IntoView {
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
            <main class="font-sans">
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/ui" view=MailPage />
                </Routes>
            </main>
        </Router>
    }
}