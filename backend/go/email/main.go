package main

import (
	"log"

	"github.com/korbkrys/echo/email/assembly"
)

func main() {
	log.Println("starting queues")

	common := assembly.Setup()
	emailSubscriber := common.Subscribers.Email

	emailSubscriber.ListenV2(common.EmailConfig)

	log.Println("queues successfully started")
}
