package server

import (
	"net"
	"net/http"
)

func New(config *Config) *http.Server {
	server := &http.Server{
		Addr:              net.JoinHostPort(config.Host, config.Port),
		ReadHeaderTimeout: config.ReadHeaderTimeout,
		WriteTimeout:      config.WriteTimeout,
		IdleTimeout:       config.IdleTimeout,
		ErrorLog:          config.ErrorLogger,
	}

	return server
}
