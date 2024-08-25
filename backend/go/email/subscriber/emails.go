package subscriber

import (
	"encoding/json"
	"net/smtp"

	"github.com/korbkrys/echo/que"
)

const queName = "email"

type emailMessage struct {
	Email string `json:"email"`
	Key   string `json:"key"`
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
			emailBody := emailMessage{}
			json.Unmarshal(message.Body, &emailBody)

			body := emailBody.Key
			from := config.address
			pass := config.password
			to := emailBody.Email

			msg := "From: " + from + "\n" +
				"To: " + to + "\n" +
				"Subject: Hello there\n\n" +
				body

			smtp.SendMail("smtp.gmail.com:587",
				smtp.PlainAuth("", from, pass, "smtp.gmail.com"),
				from, []string{to}, []byte(msg))

			message.Ack(false)
		}
	}()
	<-forever

	return nil
}
