use std::env::{self, VarError};

use futures_lite::StreamExt;

use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};

use std::str;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn_str = env::var("RABBITMQ_CONNECTION_STRING")?;
    let queue_name = env::var("RABBITMQ_QUEUE")?;

    let conn = Connection::connect(&conn_str, ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;
    let mut options = QueueDeclareOptions::default();
    options.durable = true;

    let mut consumer = channel
        .basic_consume(
            &queue_name,
            "rusted-qasa",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        println!("Received: {}", str::from_utf8(delivery.data.as_slice())?);
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }

    Ok(())
}
