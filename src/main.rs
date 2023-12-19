use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_s3 as s3;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Json, Router,
};
use mail_parser::Message;
use serde_json::{json, Value};
use dotenvy::dotenv;
use std::{collections::HashMap, fs::File, io::Read, sync::Arc, env};
use lambda_http::run;

#[derive(Clone, Debug)]
struct AppState {
    aws_config: SdkConfig,
    mail_bucket: String
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
     // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
     .with_max_level(tracing::Level::INFO)
     // disable printing the name of the module in every log line.
     .with_target(false)
     // disabling time is handy because CloudWatch will add the ingestion time.
     .without_time()
     .init();

    // dotenv().expect(".env file not found");
    let mail_bucket = env::var("MAIL_BUCKET").expect("MAIL_BUCKET not set");
    // let aws_profile_name = env::var("AWS_PROFILE").expect("AWS_PROFILE not set");

    let aws_config = aws_config::from_env()
        // .profile_name(aws_profile_name)
        .load()
        .await;

    let state = Arc::new(AppState { aws_config, mail_bucket });

    let app = Router::new()
        // .route("/", get(get_email_html))
        .route(
            "/email/:id",
            get({
                let shared_state = Arc::clone(&state);
                move |path| get_email_html(path, State(shared_state))
            }),
        )
        .route("/", get(list_email))
        // .route("/roles", get(getRole).post(reconRole))
        .with_state(state);

    // println!("Listening on 0.0.0.0:3000");

    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    
    lambda_http::run(app).await.unwrap();

    Ok(())
}

// `Html` will get a `text/html` content-type
// async fn get_html(State(state): State<Arc<AppState>>) -> Html<String> {
//     let mut file = File::open("o16sv2gj63deujs728t49t4982ghkblna4oq7781").unwrap();
//     let mut contents = vec![0];
//     file.read_to_end(&mut contents).unwrap();
//
//     let message = Message::parse(&contents).unwrap();
//     let raw_body = message.body_html(0).unwrap().to_string();
//     println!("{:#?}", raw_body);
//     Html(raw_body)
// }

async fn get_email_html(
    Path(key_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let client = s3::Client::new(&state.aws_config);
    let call = client
        .get_object()
        .bucket(&state.mail_bucket)
        .key(key_id);

    let response = call.clone().send().await.unwrap();
    let data = response.body.collect().await.expect("error reading data");
    let contents = data.into_bytes();

    let message = Message::parse(&contents).unwrap();
    let raw_body = message.body_html(0).unwrap().to_string();
    Html(raw_body)
}

async fn list_email(State(state): State<Arc<AppState>>) -> Json<Value> {
    let client = s3::Client::new(&state.aws_config);
    let call = client
        .list_objects_v2()
        .bucket(&state.mail_bucket);

    let response = call.clone().send().await.unwrap();
    let array = response.contents().unwrap();
    let parsed: Vec<String> = array.iter().map(|x| x.key.clone().unwrap()).collect();
    println!("{:#?}", parsed);
    // let json = serde_json::to_string({}).unwrap();
    // let data = response.body.collect().await.expect("error reading data");
    // let contents = data.into_bytes();
    //
    // let message = Message::parse(&contents).unwrap();
    // let raw_body = message.body_html(0).unwrap().to_string();
    Json(json!({ "data": parsed }))
    // Html("".to_owned())
}
