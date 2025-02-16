package subscriber

import (
	"os"
)

type EmailConfig struct {
	fromAddress string
	password    string
}

func NewEmailConfig() *EmailConfig {
	return &EmailConfig{
		fromAddress: os.Getenv("EMAIL_ADDRESS"),
		password:    os.Getenv("EMAIL_PASSWORD"), // not needed for mailhog
	}
}
