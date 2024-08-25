use amqprs::{
    channel::{BasicPublishArguments, Channel, QueueDeclareArguments},
    connection::Connection,
    error::Error,
    BasicProperties,
};

pub type RabbitChannel = Channel;
pub type RabbitError = Error;

#[derive(Clone)]
pub struct Que {
    connection: Connection,
}

impl Que {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub async fn create_channel(&mut self, que_name: &str) -> Result<RabbitChannel, RabbitError> {
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
        &mut self,
        channel: RabbitChannel,
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
