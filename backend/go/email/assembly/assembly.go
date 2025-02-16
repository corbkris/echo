package assembly

import (
	"log"

	"github.com/korbkrys/echo/email/subscriber"
	"github.com/korbkrys/echo/que"
)

type Common struct {
	EmailConfig *subscriber.EmailConfig
	Subscribers *subscriber.EchoSubscribers
}

func Setup() *Common {
	emailConfig := subscriber.NewEmailConfig()

	queConfig := que.NewQueConfig()
	connection, err := que.Connect(queConfig)
	if err != nil {
		log.Panicf("failed to start connection: %s", err)
	}
	basicQue := que.NewBasicQue(connection)

	subscribers := subscriber.NewEchoSubscribers(basicQue)

	return &Common{
		EmailConfig: emailConfig,
		Subscribers: subscribers,
	}
}
