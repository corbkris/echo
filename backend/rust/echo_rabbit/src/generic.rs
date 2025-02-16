use amqprs::{
    callbacks::DefaultChannelCallback,
    channel::{
        BasicPublishArguments, Channel, ExchangeDeclareArguments, ExchangeType, QueueBindArguments,
        QueueDeclareArguments,
    },
    error::Error,
    BasicProperties,
};

use crate::connection::RabbitConnection;

pub type RabbitChannel = Channel;
pub type RabbitError = Error;

pub struct Que<'a> {
    pub connection: &'a RabbitConnection,
}

impl<'a> Que<'a> {
    pub fn new(connection: &'a RabbitConnection) -> Self {
        Self { connection }
    }

    pub async fn create_channel(
        &self,
        que_name: &str,
        exchange_name: &str,
        routing_key: &str,
    ) -> Result<RabbitChannel, RabbitError> {
        let channel = match self.connection.open_channel(None).await {
            Ok(channel) => channel,
            Err(err) => return Err(err),
        };

        match channel.register_callback(DefaultChannelCallback).await {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        match channel
            .exchange_declare(
                ExchangeDeclareArguments::new(exchange_name, &ExchangeType::Direct.to_string())
                    .durable(true)
                    .finish(),
            )
            .await
        {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        match channel
            .queue_declare(QueueDeclareArguments::new(que_name).durable(true).finish())
            .await
        {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        match channel
            .queue_bind(QueueBindArguments::new(
                que_name,
                exchange_name,
                routing_key,
            ))
            .await
        {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        Ok(channel)
    }

    pub async fn publish_message(
        &self,
        channel: &RabbitChannel,
        exchange_name: &str,
        routing_key: &str,
        payload: &str,
    ) -> Result<(), RabbitError> {
        channel
            .basic_publish(
                BasicProperties::default(),
                payload.as_bytes().to_vec(),
                BasicPublishArguments::new(exchange_name, routing_key),
            )
            .await
    }

    pub async fn close_channel(channel: Channel) -> Result<(), Error> {
        channel.close().await
    }
}
