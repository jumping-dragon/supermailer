use leptos::*;

use crate::ui::components::badge::Badge;
use crate::ui::components::input::Input;
use crate::ui::components::switch::Switch;

/// Renders the home page of your application.
#[component]
pub fn MailPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, _set_count) = create_signal(50.00);

    view! {
        <div class="bg-black">
            <ProgressNav progress=count />
            <div class="flex items-center text-white">
                <div class="flex flex-col py-3 ml-5 h-screen border-white w-[600px] border-x">
                    <div class="px-3">
                        <Input />
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
                        <Card />
                        <Card />
                        <Card />
                        <Card />
                        <Card />
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
fn Card() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-y-1.5 p-6 rounded-lg border bg-zinc-950 border-zinc-800">
            <h1 class="text-2xl font-semibold">Teset Smith</h1>
            <p>[UPDATED] Need help ASAP</p>
            <p class="overflow-y-hidden text-base text-zinc-400 h-[2lh] text-ellipsis line-clamp-2">
                Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.Deploy your new project in one-click.
            </p>
            <hr class="my-2.5 w-full border-zinc-800 box-border" />
            <div class="flex justify-between">
                <Badge>badge</Badge>
                <div class="text-zinc-400">01:16 am</div>
            </div>
        </div>
    }
}
