package subscriber

import (
	"os"
)

type EmailConfig struct {
	address  string
	password string
}

func NewEmailConfig() *EmailConfig {
	return &EmailConfig{
		address:  os.Getenv("EMAIL_ADDRESS"),
		password: os.Getenv("EMAIL_PASSWORD"),
	}
}
