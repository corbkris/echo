use amqprs::{
    channel::{BasicPublishArguments, Channel, QueueDeclareArguments},
    error::Error,
    BasicProperties,
};

use crate::connection::RabbitConnection;

pub type RabbitChannel = Channel;
pub type RabbitError = Error;

pub struct Que<'a> {
    connection: &'a RabbitConnection,
}

impl<'a> Que<'a> {
    pub fn new(connection: &'a RabbitConnection) -> Self {
        Self { connection }
    }

    pub async fn create_channel(&self, que_name: &str) -> Result<RabbitChannel, RabbitError> {
        match self.connection.open_channel(None).await {
            Ok(channel) => {
                match channel
                    .queue_declare(QueueDeclareArguments::new(que_name))
                    .await
                {
                    Ok(_) => Ok(channel),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
    pub async fn publish_message(
        &self,
        channel: &RabbitChannel,
        exchange: &str,
        routing_key: &str,
        payload: &str,
    ) -> Result<(), RabbitError> {
        channel
            .basic_publish(
                BasicProperties::default(),
                payload.as_bytes().to_vec(),
                BasicPublishArguments::new(exchange, routing_key),
            )
            .await
    }

    pub async fn close_channel(channel: Channel) -> Result<(), Error> {
        channel.close().await
    }
}
