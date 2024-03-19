use aws_config::BehaviorVersion;
use aws_lambda_events::ses::{SimpleEmailEvent, SimpleEmailMessage, SimpleEmailService};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use futures::future::try_join_all;
use lambda_runtime::{Context, Error, LambdaEvent};
use serde_json::json;

use std::{fs::File, io::BufReader};

fn get_pk(input: SimpleEmailService) -> String {
    let recipient = &input.receipt.recipients[0];
    let timestamp = &input.mail.timestamp;
    let message_id = &input.mail.message_id.unwrap();
    format!("{recipient}#{timestamp}#{message_id}")
}

async fn handler(event: LambdaEvent<SimpleEmailEvent>) -> Result<(), Error> {
    let aws_config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        // .profile_name(aws_profile_name)
        .load()
        .await;
    let client = aws_sdk_dynamodb::Client::new(&aws_config);
    let table = "SupermailerTable".to_string();

    let payload = event.payload;
    let records: Vec<Mail> = payload
        .records
        .iter()
        .map(|x| Mail {
            pk: get_pk(x.ses.clone()),
            sk: x.ses.mail.clone(),
        })
        .collect();

    let calls = try_join_all(records.iter().map(|x| add_item(&client, &x, &table)))
        .await;

    match calls {
        Ok(result) => println!("Added mail {:?}", result),
        Err(error) => println!("Error adding mail {:?}", error),
    }

    Ok(())
}

#[derive(Debug)]
struct Mail {
    pk: String,
    sk: SimpleEmailMessage,
}

// TODO: Error handling
async fn add_item(
    client: &Client,
    item: &Mail,
    table: &String,
) -> Result<AttributeValue, aws_sdk_dynamodb::Error> {
    let Mail { pk, sk } = item;
    let pk = AttributeValue::S(pk.to_string());
    let sk = AttributeValue::S(json!(sk).to_string());

    let request = client
        .put_item()
        .table_name(table)
        .item("PK", pk)
        .item("SK", sk)
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld);

    println!("Executing request [{request:#?}] to add item...");

    let resp = request.send().await?;

    println!("Resp [{resp:#?}]");

    let attributes = resp.attributes().unwrap();

    let pk = attributes.get("PK").cloned().unwrap();
    let sk = attributes.get("SK").cloned().unwrap();

    println!("Added mail {:?}, {:?}", pk, sk);

    Ok(pk)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // lambda_runtime::run(service_fn(handler)).await
    let file = File::open("input_ses.json").unwrap();
    let reader = BufReader::new(file);
    let payload: SimpleEmailEvent =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");
    let event: LambdaEvent<SimpleEmailEvent> = LambdaEvent {
        payload,
        context: Context::default(),
    };
    handler(event).await
}
