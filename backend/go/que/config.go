package que

import (
	"fmt"
	"os"
	"strconv"
)

type Config struct {
	user     string
	password string
	host     string
	port     int
}

func NewQueConfig() *Config {
	port, _ := strconv.Atoi(os.Getenv("RABBIT_PORT"))
	return &Config{
		user:     os.Getenv("RABBIT_USER"),
		password: os.Getenv("RABBIT_PASSWORD"),
		host:     os.Getenv("RABBIT_HOST"),
		port:     port,
	}
}

func (c *Config) connectionString() string {
	return fmt.Sprintf("amqp:://%s:%s@%s:%d", c.user, c.password, c.host, c.port)
}
