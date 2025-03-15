package main

import (
	"log"
	"os"

	"github.com/korbkrys/echo/email/assembly"
	"github.com/sirupsen/logrus"
)

var logger = logrus.New()

func init() {
	logger.SetFormatter(&logrus.TextFormatter{
		FullTimestamp: true,
	})

	logFile, err := os.OpenFile("../../../devops/develop/loki/myapp.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		log.Fatal("Error opening log file: ", err)
	}

	logger.SetOutput(logFile)
}

func main() {
	logger.Info("starting queues")

	common := assembly.Setup()
	emailSubscriber := common.Subscribers.Email

	emailSubscriber.ListenV2(logger, common.EmailConfig)

	logger.Info("queues successfully started")
}
