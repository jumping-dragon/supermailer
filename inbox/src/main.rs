use aws_config::BehaviorVersion;
use aws_lambda_events::ses::{SimpleEmailEvent, SimpleEmailMessage, SimpleEmailService};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use futures::future::try_join_all;
use lambda_runtime::{service_fn, Context, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

use std::{fs::File, io::BufReader};

fn get_params(input: SimpleEmailService) -> (String, i64, String, String) {
    let pk = &input.receipt.recipients[0];
    let sk = &input.mail.timestamp.timestamp();
    let message_id = &input.mail.message_id.unwrap();
    let subject = &input.mail.common_headers.subject.unwrap();
    // format!("{pk}#{timestamp}#{message_id}")
    (
        pk.to_string(),
        *sk,
        message_id.to_string(),
        subject.to_string(),
    )
}

async fn handler(event: LambdaEvent<SimpleEmailEvent>) -> Result<(), Error> {
    let aws_config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .profile_name("alvinjanuar.com")
        .load()
        .await;
    let client = aws_sdk_dynamodb::Client::new(&aws_config);
    let table = "SupermailerTable".to_string();

    let payload = event.payload;
    let records: Vec<Mail> = payload
        .records
        .iter()
        .map(|x| {
            let (pk, sk, message_id, subject) = get_params(x.ses.clone());
            Mail {
                pk,
                sk,
                message_id,
                subject,
                raw: Some(x.ses.mail.clone()),
            }
        })
        .collect();

    let calls = try_join_all(
        records
            .iter()
            .map(|x| add_user_if_not_exist(&client, &x.pk, &table)),
    )
    .await;

    let calls = try_join_all(records.iter().map(|x| add_item(&client, &x, &table))).await;

    match calls {
        Err(error) => println!("Error: {:?}", error),
        Ok(_) => println!("LMD OK!"),
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mail {
    pk: String,
    sk: i64,
    message_id: String,
    subject: String,
    raw: Option<SimpleEmailMessage>,
}

// TODO: Error handling
async fn add_item(
    client: &Client,
    item: &Mail,
    table: &String,
) -> Result<String, aws_sdk_dynamodb::Error> {
    let Mail {
        pk,
        raw,
        sk,
        message_id,
        subject,
    } = item;
    let pk = AttributeValue::S(pk.to_string());
    let raw = AttributeValue::M(serde_dynamo::to_item(raw).unwrap());
    let sk = AttributeValue::N(sk.to_string());
    let message_id = AttributeValue::S(message_id.to_string());
    let subject = AttributeValue::S(subject.to_string());

    let request = client
        .put_item()
        .table_name(table)
        .item("pk", pk.clone())
        .item("raw", raw)
        .item("sk", sk)
        .item("message_id", message_id)
        .item("subject", subject)
        .return_consumed_capacity(aws_sdk_dynamodb::types::ReturnConsumedCapacity::Total);

    let resp = request.send().await?;

    let consumed_capacity = resp.consumed_capacity().unwrap();
    let capacity_units = consumed_capacity.capacity_units.unwrap();

    println!(
        "Added mail {:?} at {:?} w/ key: {:?}, used {:?} capacity_units",
        item.pk.clone(),
        item.sk.clone(),
        item.message_id.clone(),
        capacity_units
    );

    Ok(item.pk.clone())
}

// TODO: Error handling
async fn add_user_if_not_exist(
    client: &Client,
    user: &String,
    table: &String,
) -> Result<(), aws_sdk_dynamodb::Error> {
    let subject = AttributeValue::S(user.to_string());
    let user_table = "SupermailerUserTable".to_string();

    let call = client
        .query()
        .table_name(table)
        .key_condition_expression("pk = :pk")
        .projection_expression("pk, sk, subject")
        .expression_attribute_values(":pk", subject.clone())
        .limit(1);

    let resp = call.send().await?;

    if resp.count == 0 {
        let request = client
            .put_item()
            .table_name(&user_table)
            .item("pk", AttributeValue::S("USER".to_string()))
            .item("sk", subject)
            .item("message_count", AttributeValue::N("1".to_string()))
            .return_consumed_capacity(aws_sdk_dynamodb::types::ReturnConsumedCapacity::Total);

        let resp = request.send().await?;
        let consumed_capacity = resp.consumed_capacity().unwrap();
        let capacity_units = consumed_capacity.capacity_units.unwrap();

        println!(
            "Added user {:?}, used {:?} capacity_units",
            user, capacity_units
        );
    } else {
        let request = client
            .update_item()
            .table_name(&user_table)
            .key("pk", AttributeValue::S("USER".to_string()))
            .key("sk", subject)
            .update_expression("ADD message_count :one")
            .expression_attribute_values(":one", AttributeValue::N("1".to_string()))
            .return_consumed_capacity(aws_sdk_dynamodb::types::ReturnConsumedCapacity::Total);

        let resp = request.send().await?;
        let consumed_capacity = resp.consumed_capacity().unwrap();
        let capacity_units = consumed_capacity.capacity_units.unwrap();

        println!(
            "Iterated message count of user {:?}, used {:?} capacity_units",
            user, capacity_units
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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
