#[cfg(test)]
mod tests {
    use amqprs::callbacks::DefaultChannelCallback;
    use amqprs::channel::{
        BasicPublishArguments, ExchangeDeclareArguments, ExchangeType, QueueBindArguments,
        QueueDeclareArguments,
    };
    use amqprs::BasicProperties;

    use crate::connection::Config;
    use crate::generic::Que;
    use crate::models::emails::EmailSignup;

    #[tokio::test]
    async fn test_generic() {
        let connection = match Config::new().connect().await {
            Ok(connection) => connection,
            Err(err) => {
                panic!("{}", err)
            }
        };

        let que_name = "email_que";
        let exchange_name = "my_exchange";
        let routing_key = "email_key";

        let que = Que::new(&connection);

        let channel = match que.connection.open_channel(None).await {
            Ok(channel) => channel,
            Err(err) => {
                panic!("{}", err)
            }
        };

        match channel.register_callback(DefaultChannelCallback).await {
            Ok(_) => {
                println!("sucsesfully regesterd callback")
            }
            Err(err) => {
                panic!("failed to regester callback {}", err)
            }
        }

        match channel
            .exchange_declare(
                ExchangeDeclareArguments::new(exchange_name, &ExchangeType::Direct.to_string())
                    .durable(true)
                    .finish(),
            )
            .await
        {
            Ok(_) => {
                println!("exchange sucsesfully declaired")
            }
            Err(err) => {
                panic!("failed to declair exchange {}", err)
            }
        };

        match channel
            .queue_declare(QueueDeclareArguments::new(que_name).durable(true).finish())
            .await
        {
            Ok(_) => {
                println!("que succesfully declaired")
            }
            Err(err) => {
                panic!("{}", err)
            }
        };

        match channel
            .queue_bind(QueueBindArguments::new(
                que_name,
                exchange_name,
                routing_key,
            ))
            .await
        {
            Ok(_) => {
                println!("que succesfully bound")
            }
            Err(err) => {
                panic!("failed to bind queue {}", err)
            }
        };

        let email = EmailSignup::new("email@gmail.com".to_string(), "123456".to_string());
        let payload = &email.to_payload();

        channel.tx_select().await.expect("failed to start tx");

        match channel
            .basic_publish(
                BasicProperties::default(),
                payload.as_bytes().to_vec(),
                BasicPublishArguments::new(exchange_name, routing_key),
            )
            .await
        {
            Ok(_) => {
                println!("message sent")
            }
            Err(err) => {
                panic!("{}", err)
            }
        };

        channel
            .tx_commit()
            .await
            .expect("failed to commit transaction");
    }
}
