use aws_config::SdkConfig;
use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use leptos_axum::AxumRouteListing;

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
// #[derive(Debug, Clone)]
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub aws_config: SdkConfig,
    pub mail_config: MailConfig,
    pub leptos_options: LeptosOptions,
    pub routes: Vec<AxumRouteListing>,
}

#[derive(Debug, Clone)]
pub struct MailConfig {
    pub mail_bucket: String,
    pub mail_db: String,
    pub user_db: String,
}
