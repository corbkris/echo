package assembly

import (
	"github.com/korbkrys/echo/email/subscriber"
	"github.com/korbkrys/echo/que"
)

type Common struct {
	Subscribers *subscriber.EchoSubscribers
	Email       *subscriber.EmailConfig
}

func Setup() *Common {
	config := que.NewQueConfig()
	connection, err := que.Connect(config)
	if err != nil {
		panic(err)
	}
	basicQue := que.NewBasicQue(connection)

	subscribers := subscriber.NewEchoSubscribers(basicQue)
	email := subscriber.NewEmailConfig()

	return &Common{
		Email:       email,
		Subscribers: subscribers,
	}
}
