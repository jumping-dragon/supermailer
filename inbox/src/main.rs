use aws_config::BehaviorVersion;
use aws_lambda_events::ses::{SimpleEmailEvent, SimpleEmailMessage, SimpleEmailService};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use futures::future::try_join_all;
use lambda_runtime::{Context, Error, LambdaEvent, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::{collections::HashMap, fs::File, io::BufReader};

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
        Err(error) => println!("Error: {:?}", error),
        Ok(_) => println!("LMD OK!"),
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Mail {
    pk: String,
    sk: SimpleEmailMessage,
}

// TODO: Error handling
async fn add_item(
    client: &Client,
    item: &Mail,
    table: &String,
) -> Result<String, aws_sdk_dynamodb::Error> {
    let Mail { pk, sk } = item;
    let pk = AttributeValue::S(pk.to_string());
    // let sk = AttributeValue::S(json!(sk).to_string());
    let sk = AttributeValue::M(serde_dynamo::to_item(sk).unwrap());

    let request = client
        .put_item()
        .table_name(table)
        .item("PK", pk.clone())
        .item("SK", sk)
        .return_consumed_capacity(aws_sdk_dynamodb::types::ReturnConsumedCapacity::Total);

    // println!("Executing request [{request:#?}] to add item...");

    let resp = request.send().await?;

    // println!("Resp [{resp:#?}]");

    let consumed_capacity = resp.consumed_capacity().unwrap();

    let capacity_units = consumed_capacity.capacity_units.unwrap();

    println!("Added mail {:?}, used {:?} capacity_units", item.pk.clone(), capacity_units);

    Ok(item.pk.clone())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(debug_assertions)]
    {
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
    #[cfg(not(debug_assertions))]
    {
        lambda_runtime::run(service_fn(handler)).await
    }
}
