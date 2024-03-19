package main

import (
	"context"
	server "gallery/server"
	"gallery/utils/logger"
	"log/slog"
	"net/http"
	"os"
)

func main() {
	srv := setupServer()

	sysLogger := logger.NewSLogger(os.Stdout, slog.LevelDebug)
	ctx := context.Background()

	sysLogger.LogAttrs(
		ctx,
		slog.LevelInfo,
		"The server starts running at",
		slog.String("address", srv.Addr),
	)

	sysLogger.LogAttrs(ctx, slog.LevelError, srv.ListenAndServe().Error())
	os.Exit(1)
}

func setupServer() *http.Server {

	srvConfig := server.NewConfig()

	srvConfig.Init(
		"127.0.0.1",
		"8080",
		"1m",
		"1m",
		"5m",
		logger.NewServerErrorLogger(os.Stderr, slog.LevelError),
	)

	return server.New(srvConfig)
}
