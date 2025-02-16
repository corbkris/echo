package subscriber

import (
	"encoding/json"
	"log"

	"github.com/korbkrys/echo/que"
	"gopkg.in/gomail.v2"
)

const (
	queName      = "email_que"
	exchangeName = "my_exchange"
	routingKey   = "email_key"
)

type emailMessage struct {
	ToEmail         string `json:"email"`
	VerificationKey string `json:"key"` // this is the verification key we are sending to the client.
}

type EmailSubscriber struct {
	que *que.BasicQue
}

func NewEmailSubscriber(que *que.BasicQue) *EmailSubscriber {
	return &EmailSubscriber{
		que: que,
	}
}

func (e *EmailSubscriber) Listen(config *EmailConfig) error {
	channel, err := e.que.DeclareChannel()
	if err != nil {
		return err
	}
	que, err := e.que.DeclareQue(queName, channel)
	if err != nil {
		return err
	}

	messages, err := e.que.GetMessages(channel, que)
	if err != nil {
		return err
	}

	forever := make(chan bool)
	go func() {
		for message := range messages {
			emailMessage := emailMessage{}
			if err := json.Unmarshal(message.Body, &emailMessage); err != nil {
				log.Printf("failed to unmarshal message: %s", err)
				continue
			}

			mail := gomail.NewMessage()
			mail.SetHeader("From", config.fromAddress)
			mail.SetHeader("To", emailMessage.ToEmail)
			mail.SetHeader("Subject", "ECHO Email Verification Key")
			mail.SetBody("text/plain", emailMessage.VerificationKey)

			dialer := gomail.NewDialer("localhost", 1025, "", "")

			// Send email
			if err := dialer.DialAndSend(mail); err != nil {
				log.Printf("failed to send email: %s", err)
				continue
			}
			log.Println("email successfully sent")

			message.Ack(false)
		}
	}()
	<-forever

	return nil
}

func (e *EmailSubscriber) ListenV2(config *EmailConfig) {
	channel, err := e.que.Connection.Channel()
	if err != nil {
		e.que.Connection.Close()
		panic(err)
	}
	/*
		defer func() {
			channel.Close()
			e.que.Connection.Close()
		}()
	*/
	if err := channel.ExchangeDeclare(exchangeName, "direct", true, false, false, false, nil); err != nil {
		panic(err)
	}

	que, err := channel.QueueDeclare(queName, true, false, false, false, nil)
	if err != nil {
		panic(err)
	}

	if err := channel.QueueBind(que.Name, routingKey, exchangeName, false, nil); err != nil {
		panic(err)
	}

	messages, err := channel.Consume(que.Name, "", false, false, false, false, nil)
	if err != nil {
		panic(err)
	}
	log.Println("everything started")

	forever := make(chan bool)
	go func() {
		for message := range messages {
			log.Println("new message")
			log.Println(message)

			emailMessage := emailMessage{}
			if err := json.Unmarshal(message.Body, &emailMessage); err != nil {
				log.Printf("failed to unmarshal message: %s", err)
				message.Nack(false, false)
				continue
			}

			mail := gomail.NewMessage()
			mail.SetHeader("From", config.fromAddress)
			mail.SetHeader("To", emailMessage.ToEmail)
			mail.SetHeader("Subject", "ECHO Email Verification Key")
			mail.SetBody("text/plain", emailMessage.VerificationKey)
			log.Println(emailMessage.VerificationKey)

			dialer := gomail.NewDialer("localhost", 1025, "", "")

			// Send email
			if err := dialer.DialAndSend(mail); err != nil {
				message.Nack(false, true)
				log.Printf("failed to send email: %s", err)
				continue
			}
			log.Println("email successfully sent")

			message.Ack(false)
			// message.Nack(false, true)
		}
	}()
	log.Println("running...")
	<-forever
}
