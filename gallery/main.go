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
	execLogger := logger.NewSLogger(os.Stdout, slog.LevelDebug)
	logCtx := context.Background()
	srv := setupServer()

	execLogger.LogAttrs(
		logCtx,
		slog.LevelInfo,
		"The server starts running at",
		slog.String("address", srv.Addr),
	)

	execLogger.Info(server.Run(logCtx, execLogger, srv).Error())
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
		os.Stdout,
	)

	return server.New(srvConfig)
}
