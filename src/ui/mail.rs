use leptos::logging;
use leptos::prelude::*;
use leptos_router::hooks::query_signal;

use crate::api_types::{ListEmailsResponse, ListUsersResponse, Mail};
use crate::ui::components::badge::Badge;
use crate::ui::components::input::Input;
use crate::ui::components::switch::Switch;
use chrono::{Duration, Utc};

#[server(ListEmails, "/api_fn")]
pub async fn list_emails_fn(email: String) -> Result<ListEmailsResponse, ServerFnError> {
    use crate::api::list_emails;
    use crate::state::AppState;
    let state = use_context::<AppState>();

    match state {
        Some(state) => Ok(list_emails(state, email).await),
        None => Err(ServerFnError::ServerError("error_state".to_string())),
    }
}

#[server(ListUsers, "/api_fn")]
pub async fn list_users_fn() -> Result<ListUsersResponse, ServerFnError> {
    use crate::api::list_users;
    use crate::state::AppState;
    let state = use_context::<AppState>();

    match state {
        Some(state) => Ok(list_users(state).await),
        None => Err(ServerFnError::ServerError("error_state".to_string())),
    }
}

// #[server(GetEmailHtml, "/api_fn")]
// pub async fn get_email_html_fn(key_id: String) -> Result<String, ServerFnError> {
//     use crate::api::get_email_html;
//     use crate::state::AppState;
//     let state = use_context::<AppState>();
//
//     match state {
//         Some(state) => Ok(get_email_html(key_id, state).await),
//         None => Err(ServerFnError::ServerError("error_state".to_string())),
//     }
// }

/// Renders the home page of your application.
#[component]
pub fn MailPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, _set_count) = signal(50.00);
    let (email, set_email) = query_signal::<String>("e");
    // let (current_showing, _set_current_showing) = signal("".to_string());

    let users = Resource::new(
        move || count.get(),
        move |_value| async move { list_users_fn().await },
    );

    let mails = Resource::new(
        move || email.get(),
        move |value| async move {
            list_emails_fn(value.unwrap_or("web@alvinjanuar.com".to_string())).await
            // TODO:
            // Change hardcoded value to first user
        },
    );

    // let showing = Resource::new(
    //     move || current_showing.get(),
    //     move |value| async move { get_email_html_fn(value.to_string()).await },
    // );

    view! {
        <div class="bg-black">
            <ProgressNav progress=count />
            <div class="flex items-center text-white">
                <div class="flex flex-col flex-grow py-3 ml-5 h-screen border-white shrink-0 w-[600px] border-x">
                    <div class="flex flex-col gap-y-3 px-3">
                        <Input />
                        // <input
                        // type="range"
                        // max="100"
                        // value=count
                        // on:change=move |event| {
                        // set_count(event_target_value(&event).parse::<f64>().unwrap());
                        // }
                        // />
                        <Suspense fallback=move || {
                            view! { <p>Loading...</p> }
                        }>
                            {move || match users.get() {
                                None => view! { <p>"Loading..."</p> }.into_any(),
                                Some(data) => {
                                    match data {
                                        Ok(api) => {
                                            view! {
                                                <select on:change=move |ev| {
                                                    let new_value = event_target_value(&ev);
                                                    logging::log!("{}", &new_value);
                                                    set_email.set(Some(new_value.parse::<String>().unwrap()));
                                                }>
                                                    <For
                                                        // a function that returns the items we're iterating over; a signal is fine
                                                        each=move || api.data.clone()
                                                        // a unique key for each item
                                                        key=|user| user.sk.clone()
                                                        // renders each item to a view
                                                        children=move |user| {
                                                            let e = email.get();
                                                            let is_current = user.clone().sk
                                                                == e.unwrap_or("web@alvinjanuar.com".to_string());
                                                            view! {
                                                                <option
                                                                    class=("bg-white", is_current)
                                                                    value=user.sk.clone()
                                                                    selected=is_current
                                                                >
                                                                    {user.sk.clone()}
                                                                </option>
                                                            }
                                                        }
                                                    />
                                                </select>
                                            }
                                                .into_any()
                                        }
                                        Err(e) => view! { <p>{e.to_string()}</p> }.into_any(),
                                    }
                                }
                            }}
                        </Suspense>
                    // </select>
                    </div>
                    <div class="relative my-1">
                        <div class="absolute left-0 -translate-x-1/2">
                            <Suspense fallback=move || {
                                view! { <Badge>...</Badge> }
                            }>
                                {move || match mails.get() {
                                    None => view! { <Badge>...</Badge> }.into_any(),
                                    Some(data) => {
                                        match data {
                                            Ok(api) => {
                                                view! { <Badge>{api.data.len()}</Badge> }.into_any()
                                            }
                                            Err(e) => view! { <p>{e.to_string()}</p> }.into_any(),
                                        }
                                    }
                                }}
                            </Suspense>
                        </div>
                        <div class="absolute right-0 translate-x-1/2">
                            <Switch />
                        </div>
                        <hr class="mt-2.5 w-full border-zinc-800 box-border" />
                    </div>
                    <div class="flex overflow-y-auto flex-col gap-y-3 px-3 pt-3 w-full">
                        <Transition fallback=move || {
                            view! { <p>"Loading..."</p> }
                        }>
                            {move || match mails.get() {
                                None => view! { <p>"No Data"</p> }.into_any(),
                                Some(data) => {
                                    match data {
                                        Ok(api) => {
                                            view! {
                                                <div>
                                                    <For
                                                        // a function that returns the items we're iterating over; a signal is fine
                                                        each=move || api.data.clone()
                                                        // a unique key for each item
                                                        key=|mail| mail.sk
                                                        // renders each item to a view
                                                        children=move |mail| {
                                                            view! { <Card mail=mail /> }
                                                        }
                                                    />
                                                </div>
                                            }
                                                .into_any()
                                        }
                                        Err(e) => view! { <p>{e.to_string()}</p> }.into_any(),
                                    }
                                }
                            }}
                        </Transition>
                    </div>
                </div>
                <div class="hidden flex-col flex-grow py-6 px-8 h-screen sm:flex">
                    <h1 class="text-2xl font-semibold">Teset Smith</h1>
                    <p>[UPDATED] Need help ASAP</p>
                    <div class="flex justify-between">
                        <Badge>badge</Badge>
                        <div class="text-zinc-400">01:16 am</div>
                    </div>
                    <hr class="my-2.5 w-full border-zinc-800 box-border" />
                // <p class="overflow-y-hidden text-base">
                // Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.
                // </p>
                //
                // <p class="overflow-y-hidden text-base">{move || showing.get()}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProgressNav(progress: ReadSignal<f64>) -> impl IntoView {
    let percentage = move || progress.get() / 100.0;
    view! {
        <div class="fixed top-0 right-0 left-0 h-0.5">
            <div
                class="w-full h-px bg-white transition-all origin-left"
                style:transform=move || format!("scaleX({})", percentage())
            />
        </div>
    }
}

#[component]
fn Card(mail: Mail) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-y-1.5 p-6 rounded-lg border bg-zinc-950 border-zinc-800">
            <h1 class="text-2xl font-semibold">{mail.from}</h1>
            <p>{mail.subject}</p>
            <p class="overflow-y-hidden text-base text-zinc-400 h-[2lh] text-ellipsis line-clamp-2">
                {mail.first_sentence}
            </p>
            <hr class="my-2.5 w-full border-zinc-800 box-border" />
            <div class="flex justify-between">
                <Badge>badge</Badge>
                <a href="/api/email/".to_string() + &mail.message_id target="_blank">
                    Open Mail
                </a>
                <div class="text-zinc-400">
                    <RelativeTime timestamp=mail.sk />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RelativeTime(#[prop(into)] timestamp: i64) -> impl IntoView {
    let relative_time = move || {
        let now = Utc::now().timestamp();
        let diff = Duration::seconds(now - timestamp);

        match diff {
            d if d < Duration::seconds(60) => "just now".to_string(),
            d if d < Duration::minutes(60) => format!("{} minutes ago", d.num_minutes()),
            d if d < Duration::hours(24) => format!("{} hours ago", d.num_hours()),
            d if d < Duration::days(30) => format!("{} days ago", d.num_days()),
            d if d < Duration::days(365) => format!("{} months ago", d.num_days() / 30),
            _ => format!("{} years ago", diff.num_days() / 365),
        }
    };

    view! { <span>{relative_time}</span> }
}
