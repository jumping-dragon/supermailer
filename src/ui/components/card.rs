use leptos::prelude::*;
use crate::api_types::Mail;
use crate::ui::components::badge::Badge;
use chrono::{Duration, Utc};

#[component]
pub fn Card(mail: Mail) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-y-1.5 p-5 sm:p-6 rounded-lg border bg-zinc-950 border-zinc-800">
            <h1 class="text-lg sm:text-2xl font-semibold line-clamp-2">{mail.from}</h1>
            <p>{mail.subject}</p>
            <p class="overflow-y-hidden text-sm sm:text-base text-zinc-400 h-[3lh] sm:h-[2lh] text-ellipsis line-clamp-3 sm:line-clamp-2">
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
pub fn CardLoading() -> impl IntoView {
        view! {
            <div class="flex flex-col gap-y-1.5 p-5 sm:p-6 rounded-lg border bg-zinc-800 animate-pulse min-h-40 border-zinc-800" />
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
