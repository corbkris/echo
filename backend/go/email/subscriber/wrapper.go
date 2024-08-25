package subscriber

import "github.com/korbkrys/echo/que"

type EchoSubscribers struct {
	Email *EmailSubscriber
}

func NewEchoSubscribers(que *que.BasicQue) *EchoSubscribers {
	return &EchoSubscribers{
		Email: NewEmailSubscriber(que),
	}
}
