use crate::api_types::{ListEmailsResponse, ListUsersResponse, Mail, User};
use crate::state::AppState;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_s3 as s3;
use axum::{
    extract::{Path, State},
    response::Html,
    Json,
};
use dynamodb::types::AttributeValue;
use mail_parser::Message;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// use leptos::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    mail_db: String,
    id: String,
}

// #[server(GetEmailHtml, "/api_fn", "GetJson", "email")]
// pub async fn test_sfn(
//     id: String,
// ) -> Result<Test, ServerFnError> {

//     let state: AppState = use_context::<AppState>()
//         .ok_or(ServerFnError::ServerError("No server state".to_string()))?;

//     let mail_db = state.mail_config.mail_db;

//     let ret = Test {
//         mail_db,
//         id
//     };

//     println!("Æ");
//     Ok(ret)
// }

pub async fn get_email_html(
    Path(key_id): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {
    let client = s3::Client::new(&state.aws_config);
    let call = client
        .get_object()
        .bucket(&state.mail_config.mail_bucket)
        .key(key_id);

    let response = call.clone().send().await.unwrap();
    let data = response.body.collect().await.expect("error reading data");
    let contents = data.into_bytes();

    let message = Message::parse(&contents).unwrap();
    let raw_body = message.body_html(0).unwrap().to_string();
    Html(raw_body)
}

pub async fn list_emails_api(State(state): State<AppState>) -> Json<ListEmailsResponse> {
    // let client = s3::Client::new(&state.aws_config);
    // let call = client.list_objects_v2().bucket(&state.mail_bucket);
    //
    // let response = call.clone().send().await.unwrap();
    // let array = response.contents();
    // let parsed: Vec<String> = array.iter().map(|x| x.key.clone().unwrap()).collect();
    // println!("{:#?}", parsed);
    let response = list_emails(state).await;
    Json(response)
}

pub async fn list_emails(state: AppState) -> ListEmailsResponse {
    let _client = dynamodb::Client::new(&state.aws_config);
    let call = _client
        .query()
        .table_name(&state.mail_config.mail_db)
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
    ListEmailsResponse { data: mails }
}

pub async fn list_users(state: AppState) -> ListUsersResponse {
    let _client = dynamodb::Client::new(&state.aws_config);
    let call = _client
        .query()
        .table_name(&state.mail_config.user_db)
        .key_condition_expression("pk = :pk")
        .projection_expression("pk, sk, message_count")
        .expression_attribute_values(":pk", AttributeValue::S("USER".to_string()))
        .limit(20);

    let resp = call.send().await.unwrap();
    let users: Vec<User> = resp
        .items()
        .iter()
        .map(|x| User {
            pk: x.get("pk").unwrap().as_s().unwrap().to_string(),
            sk: x.get("sk").unwrap().as_s().unwrap().to_string(),
            message_count: x
                .get("message_count")
                .unwrap()
                .as_n()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
        })
        .collect();
    ListUsersResponse { data: users }
}
