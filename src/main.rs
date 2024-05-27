use anyhow::Result;
use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_s3 as s3;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Json, Router,
};
use dotenvy::dotenv;
use dynamodb::types::AttributeValue;
use mail_parser::Message;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{env, sync::Arc};

#[derive(Clone, Debug)]
struct AppState {
    aws_config: SdkConfig,
    mail_bucket: String,
    mail_db: String,
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
    #[cfg(debug_assertions)]
    {
        dotenv().expect(".env file not found");
    }
    let mail_bucket = env::var("MAIL_BUCKET").expect("MAIL_BUCKET not set");
    let mail_db = env::var("MAIL_DB").expect("MAIL_DB not set");
    // let aws_profile_name = env::var("AWS_PROFILE").expect("AWS_PROFILE not set");

    let aws_config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        // .profile_name(aws_profile_name)
        .load()
        .await;

    let state = Arc::new(AppState {
        aws_config,
        mail_bucket,
        mail_db,
    });

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

    #[cfg(debug_assertions)]
    {
        println!("Listening on http://localhost:3001");
        axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
    #[cfg(not(debug_assertions))]
    {
        lambda_http::run(app).await.unwrap();
    }
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
    let call = client.get_object().bucket(&state.mail_bucket).key(key_id);

    let response = call.clone().send().await.unwrap();
    let data = response.body.collect().await.expect("error reading data");
    let contents = data.into_bytes();

    let message = Message::parse(&contents).unwrap();
    let raw_body = message.body_html(0).unwrap().to_string();
    Html(raw_body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mail {
    pk: String,
    sk: i64,
    message_id: String,
    subject: String,
    from: Vec<String>,
}

async fn list_email(State(state): State<Arc<AppState>>) -> Json<Value> {
    // let client = s3::Client::new(&state.aws_config);
    // let call = client.list_objects_v2().bucket(&state.mail_bucket);
    //
    // let response = call.clone().send().await.unwrap();
    // let array = response.contents();
    // let parsed: Vec<String> = array.iter().map(|x| x.key.clone().unwrap()).collect();
    // println!("{:#?}", parsed);

    let _client = dynamodb::Client::new(&state.aws_config);
    let call = _client
        .query()
        .table_name(&state.mail_db)
        .key_condition_expression("pk = :pk")
        .projection_expression("pk, message_id, sk, subject, #r.#ch.#f")
        .expression_attribute_names("#r", "raw")
        .expression_attribute_names("#ch", "commonHeaders")
        .expression_attribute_names("#f", "from")
        .expression_attribute_values(":pk", AttributeValue::S("web@alvinjanuar.com".to_string()))
        .limit(20);

    let resp = call.send().await.unwrap();
    let mails: Vec<Mail> = resp
        .items()
        .iter()
        .map(|x| Mail {
            pk: x.get("pk").unwrap().as_s().unwrap().to_string(),
            sk: x.get("sk").unwrap().as_n().unwrap().parse::<i64>().unwrap(),
            message_id: x.get("message_id").unwrap().as_s().unwrap().to_string(),
            subject: x.get("subject").unwrap().as_s().unwrap().to_string(),
            from: x
                .get("raw")
                .unwrap()
                .as_m()
                .unwrap()
                .get("commonHeaders")
                .unwrap()
                .as_m()
                .unwrap()
                .get("from")
                .unwrap()
                .as_l()
                .unwrap()
                .to_owned()
                .iter()
                .map(|x| x.as_s().unwrap().to_owned())
                .collect::<Vec<String>>(),
        })
        .collect();
    Json(json!({ "data": mails }))
    // Html("".to_owned())
}
