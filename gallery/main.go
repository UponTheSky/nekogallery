package main

import (
	server "gallery/server"
	"log"
	"net/http"
)

func main() {
	srv := setupServer()

	log.Fatal(srv.ListenAndServe())
}

func setupServer() *http.Server {
	srvConfig := server.NewConfig()
	srvConfig.Init(
		"127.0.0.1",
		"8080",
		"1m",
		"1m",
		"5m",
		&log.Logger{},
	)

	return server.New(srvConfig)
}
