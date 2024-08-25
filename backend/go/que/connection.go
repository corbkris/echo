package que

import (
	"github.com/rabbitmq/amqp091-go"
)

func Connect(config *Config) (connection *amqp091.Connection, err error) {
	return amqp091.Dial(config.connectionString())
}

type BasicQue struct {
	connection *amqp091.Connection
}

func NewBasicQue(connection *amqp091.Connection) *BasicQue {
	return &BasicQue{
		connection: connection,
	}
}

func (c *BasicQue) DeclareChannel() (*amqp091.Channel, error) {
	return c.connection.Channel()
}

func (c *BasicQue) DeclareQue(que_name string, channel *amqp091.Channel) (amqp091.Queue, error) {
	return channel.QueueDeclare(que_name, false, false, false, false, nil)
}

func (c *BasicQue) GetMessages(channel *amqp091.Channel, que amqp091.Queue) (<-chan amqp091.Delivery, error) {
	return channel.Consume(que.Name, "", true, false, false, false, nil)
}
