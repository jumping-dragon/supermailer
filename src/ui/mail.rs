use leptos::*;

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
        Some(state) => Ok(list_emails(state).await),
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

/// Renders the home page of your application.
#[component]
pub fn MailPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(50.00);

    let users = create_local_resource(
        count,
        // every time `count` changes, this will run
        move |value| async move { list_users_fn().await },
    );

    let mails = create_local_resource(
        count,
        // every time `count` changes, this will run
        move |value| async move {
            println!("count {}", count());
            list_emails_fn(value.to_string()).await
        },
    );

    view! {
        <div class="bg-black">
            <ProgressNav progress=count />
            <div class="flex items-center text-white">
                <div class="flex flex-col py-3 ml-5 h-screen border-white w-[600px] border-x">
                    <div class="px-3">
                        <Input />
                        <input
                            type="range"
                            max="100"
                            value=count
                            on:change=move |event| {
                                set_count(event_target_value(&event).parse::<f64>().unwrap());
                            }
                        />
                    </div>
                    <div class="relative my-1">
                        <div class="absolute left-0 -translate-x-1/2">
                            <Badge>8</Badge>
                        </div>
                        <div class="absolute right-0 translate-x-1/2">
                            <Switch />
                        </div>
                        <hr class="mt-2.5 w-full border-zinc-800 box-border" />
                    </div>
                    <div class="flex overflow-y-auto flex-col gap-y-3 px-3 pt-3 w-full">
                        {move || match users.get() {
                            None => view! { <p>"Loading..."</p> }.into_view(),
                            Some(data) => {
                                match data {
                                    Ok(api) => {
                                        api.data
                                            .into_iter()
                                            .map(|user| { view! { <div>{user.sk}</div> }.into_view() })
                                            .collect()
                                    }
                                    Err(e) => view! { <p>{e.to_string()}</p> }.into_view(),
                                }
                            }
                        }}
                        {move || match mails.get() {
                            None => view! { <p>"Loading..."</p> }.into_view(),
                            Some(data) => {
                                match data {
                                    Ok(api) => {
                                        api.data
                                            .into_iter()
                                            .map(|mail| { view! { <Card mail=mail /> }.into_view() })
                                            .collect()
                                    }
                                    Err(e) => view! { <p>{e.to_string()}</p> }.into_view(),
                                }
                            }
                        }}
                    </div>
                </div>
                <div class="flex flex-col flex-grow py-6 px-8 h-screen">
                    <h1 class="text-2xl font-semibold">Teset Smith</h1>
                    <p>[UPDATED] Need help ASAP</p>
                    <div class="flex justify-between">
                        <Badge>badge</Badge>
                        <div class="text-zinc-400">01:16 am</div>
                    </div>
                    <hr class="my-2.5 w-full border-zinc-800 box-border" />
                    <p class="overflow-y-hidden text-base">
                        Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProgressNav(progress: ReadSignal<f64>) -> impl IntoView {
    let percentage = move || progress() / 100.0;
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
                Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.
            </p>
            <hr class="my-2.5 w-full border-zinc-800 box-border" />
            <div class="flex justify-between">
                <Badge>badge</Badge>
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
