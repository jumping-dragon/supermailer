// use aws_sdk_dynamodb as dynamodb;
// use aws_sdk_s3 as s3;
use axum::{
    extract::{Path, State},
    response::Html,
    Json,
};
// use dynamodb::types::AttributeValue;
// use mail_parser::{parsers::fields::raw, Message};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// use std::sync::Arc;
use crate::state::AppState;
use leptos::*;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    mail_db: String
}

#[server(GetEmailHtml, "/api", "GetJson", "email")]
pub async fn get_email_html(
    id: String,
    // test: Test
) -> Result<Test, ServerFnError> {
    println!("{}", id);

    let state: AppState = use_context::<AppState>()
        .ok_or(ServerFnError::ServerError("No server state".to_string()))?;

    let mail_db = state.mail_config.mail_db;

    let ret = Test {
        mail_db
    };
// pub async fn get_email_html(
//     Path(key_id): Path<String>,
//     State(state): State<AppState>,
// ) -> Html<String> {
//     let client = s3::Client::new(&state.aws_config);
//     let call = client.get_object().bucket(&state.mail_config.mail_bucket).key(key_id);

//     let response = call.clone().send().await.unwrap();
//     let data = response.body.collect().await.expect("error reading data");
//     let contents = data.into_bytes();

//     let message = Message::parse(&contents).unwrap();
//     let raw_body = message.body_html(0).unwrap().to_string();
    Ok(ret)
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Mail {
//     pk: String,
//     sk: i64,
//     message_id: String,
//     subject: String,
//     from: Vec<String>,
// }

// pub async fn list_email(State(state): State<Arc<AppState>>) -> Json<Value> {
//     // let client = s3::Client::new(&state.aws_config);
//     // let call = client.list_objects_v2().bucket(&state.mail_bucket);
//     //
//     // let response = call.clone().send().await.unwrap();
//     // let array = response.contents();
//     // let parsed: Vec<String> = array.iter().map(|x| x.key.clone().unwrap()).collect();
//     // println!("{:#?}", parsed);

//     let _client = dynamodb::Client::new(&state.aws_config);
//     let call = _client
//         .query()
//         .table_name(&state.mail_config.mail_db)
//         .key_condition_expression("pk = :pk")
//         .projection_expression("pk, message_id, sk, subject, #r.#ch.#f")
//         .expression_attribute_names("#r", "raw")
//         .expression_attribute_names("#ch", "commonHeaders")
//         .expression_attribute_names("#f", "from")
//         .expression_attribute_values(":pk", AttributeValue::S("web@alvinjanuar.com".to_string()))
//         .limit(20);

//     let resp = call.send().await.unwrap();
//     let mails: Vec<Mail> = resp
//         .items()
//         .iter()
//         .map(|x| Mail {
//             pk: x.get("pk").unwrap().as_s().unwrap().to_string(),
//             sk: x.get("sk").unwrap().as_n().unwrap().parse::<i64>().unwrap(),
//             message_id: x.get("message_id").unwrap().as_s().unwrap().to_string(),
//             subject: x.get("subject").unwrap().as_s().unwrap().to_string(),
//             from: x
//                 .get("raw")
//                 .unwrap()
//                 .as_m()
//                 .unwrap()
//                 .get("commonHeaders")
//                 .unwrap()
//                 .as_m()
//                 .unwrap()
//                 .get("from")
//                 .unwrap()
//                 .as_l()
//                 .unwrap()
//                 .to_owned()
//                 .iter()
//                 .map(|x| x.as_s().unwrap().to_owned())
//                 .collect::<Vec<String>>(),
//         })
//         .collect();
//     Json(json!({ "data": mails }))
//     // Html("".to_owned())
// }
