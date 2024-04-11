package server

import (
	"io"
	"log"
	"time"
)

type Config struct {
	Host              string
	Port              string
	ReadHeaderTimeout time.Duration
	WriteTimeout      time.Duration
	IdleTimeout       time.Duration
	ErrorLogger       *log.Logger
	LoggerWriter      io.Writer
}

func NewConfig() *Config {
	return &Config{}
}

func (c *Config) Init(
	host,
	port,
	readHeaderTimeout,
	writeTimeout,
	idleTimeout string,
	errorLogger *log.Logger,
	loggerWriter io.Writer,
) {
	c.Host = host
	c.Port = port

	rhTimeout, err := time.ParseDuration(readHeaderTimeout)

	if err != nil {
		log.Fatal(err)
	}
	c.ReadHeaderTimeout = rhTimeout

	wTimeout, err := time.ParseDuration(writeTimeout)

	if err != nil {
		log.Fatal(err)
	}
	c.WriteTimeout = wTimeout

	iTimeout, err := time.ParseDuration(idleTimeout)

	if err != nil {
		log.Fatal(err)
	}
	c.IdleTimeout = iTimeout

	c.ErrorLogger = errorLogger

	c.LoggerWriter = loggerWriter
}
