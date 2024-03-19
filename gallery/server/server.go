package server

import (
	"gallery/server/middleware"
	"net"
	"net/http"
)

func New(config *Config) *http.Server {

	var multiplexer http.Handler = http.NewServeMux()

	// register controllers(routers)
	// TODO: delete this mock router
	// TODO: add controllers with a "global" logger
	http.Handle("GET /", multiplexer)

	// register middlewares
	multiplexer = middleware.NewLogMiddleware(multiplexer)

	return &http.Server{
		Addr:              net.JoinHostPort(config.Host, config.Port),
		ReadHeaderTimeout: config.ReadHeaderTimeout,
		WriteTimeout:      config.WriteTimeout,
		IdleTimeout:       config.IdleTimeout,
		ErrorLog:          config.ErrorLogger,
		MaxHeaderBytes:    1 << 20,
		Handler:           multiplexer,
	}
}
