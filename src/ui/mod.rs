use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title, Link};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <Ui/>
            </body>
        </html>
    }
}

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
        <Router>
            <main class="font-sans">
                // <Routes fallback=|| {
                //     let mut outside_errors = Errors::default();
                //     outside_errors.insert_with_default_key(AppError::NotFound);
                //     view! { <ErrorTemplate outside_errors /> }.into_view()
                // }>
                <Routes fallback=|| "Page not found.".into_view() >
                    <Route path=StaticSegment("/") view=HomePage/>
                    <Route path=StaticSegment("/ui") view=MailPage/>
                </Routes>
            </main>
        </Router>
    }
}