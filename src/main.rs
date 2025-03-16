use cfg_if::cfg_if;
// boilerplate to run in different modes
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use aws_config::BehaviorVersion;
        use axum::{
            body::Body as AxumBody,
            extract::{Path, State},
            http::Request,
            response::{IntoResponse, Response},
            routing::get,
            Router,
        };
        use dotenvy::dotenv;
        use leptos::{
            config::get_configuration, prelude::provide_context,
        };
        use leptos_axum::{generate_route_list_with_exclusions, handle_server_fns_with_context, LeptosRoutes};
        use std::env;
        use supermailer::state::{AppState, MailConfig};
        use supermailer::{ui::*};
        use supermailer::api::{list_emails_api, get_email_html_api};

        async fn server_fn_handler(
            State(app_state): State<AppState>,
            path: Path<String>,
            request: Request<AxumBody>
        ) -> impl IntoResponse {

            println!("{:?}", path);

            handle_server_fns_with_context(move || {
                // provide_context(cx, auth_session.clone());
                provide_context(app_state.clone());
            }, request).await
        }

        async fn leptos_routes_handler(
            state: State<AppState>,
            req: Request<AxumBody>,
        ) -> Response {
            let State(app_state) = state.clone();
            let leptos_options = state.leptos_options.clone();
            let handler = leptos_axum::render_route_with_context(
                app_state.routes.clone(),
                move || {
                    // provide_context(auth_session.clone());
                    provide_context(app_state.clone());
                },
                move || shell(leptos_options.clone()),
            );
            handler(state, req).await.into_response()
        }

        #[tokio::main]
        async fn main() {
            simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

            #[cfg(debug_assertions)]
            {
                dotenv().expect(".env file not found");
            }
            let mail_bucket = env::var("MAIL_BUCKET").expect("MAIL_BUCKET not set");
            let mail_db = env::var("MAIL_DB").expect("MAIL_DB not set");
            let user_db = env::var("USER_DB").expect("USER_DB not set");
            // let aws_profile_name = env::var("AWS_PROFILE").expect("AWS_PROFILE not set");

            let aws_config = aws_config::defaults(BehaviorVersion::v2025_01_17())
                // .profile_name(aws_profile_name)
                .load()
                .await;

            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            // For deployment these variables are:
            // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
            // Alternately a file can be specified such as Some("Cargo.toml")
            // The file would need to be included with the executable when moved to deployment
            let conf = get_configuration(None).unwrap();
            let leptos_options = conf.leptos_options;
            // We don't use an address for the lambda function
            #[allow(unused_variables)]
            let addr = leptos_options.site_addr;
            let routes = generate_route_list_with_exclusions(Ui, Some(vec!["/api/".to_string(), "/api".to_string()]));

            let mail_config = MailConfig {
                mail_bucket,
                mail_db,
                user_db
            };

            let state = AppState {
                aws_config,
                mail_config,
                leptos_options,
                routes: routes.clone(),
            };

            let api_route = Router::new()
                .route("/:email", get(list_emails_api))
                .route("/email/:id", get(get_email_html_api))
                .with_state(state.clone());

            // build our application with a route
            let app = Router::new()
                .nest("/api", api_route)
                .route("/api_fn/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
                .with_state(state);

            // In development, we use the Hyper server
            #[cfg(debug_assertions)]
            {
                log::info!("listening on http://{}", &addr);
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
                // logging::log!("listening on http://{}", &addr);
                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            }

            // In release, we use the lambda_http crate
            #[cfg(not(debug_assertions))]
            {
                let app = tower::ServiceBuilder::new()
                    .layer(axum_aws_lambda::LambdaLayer::default())
                    .service(app);

                lambda_http::run(app).await.unwrap();
            }
        }
    }// client-only stuff for Trunk
    else {
        pub fn main() {
            // This example cannot be built as a trunk standalone CSR-only app.
            // Only the server may directly connect to the database.
        }
    }
}
