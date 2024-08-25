package email

import (
	"log"

	"github.com/korbkrys/echo/email/assembly"
)

func main() {
	log.Println("hello")
	common := assembly.Setup()
	emailSubscriber := common.Subscribers.Email

	emailSubscriber.Listen(common.Email)
}
