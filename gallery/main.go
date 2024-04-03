package main

import (
	"context"
	server "gallery/server"
	"gallery/utils/logger"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
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

	execLogger.Info(runServer(logCtx, execLogger, srv).Error())
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

// graceful shutdown
// see the example: https://pkg.go.dev/net/http#Server.Shutdown
func runServer(logCtx context.Context, logger *slog.Logger, srv *http.Server) error {
	// to wait for the goroutine to finish before the main thread
	idleConnsClosed := make(chan struct{})

	go func() {
		// waiting for the interrupt signal to come
		sigChan := make(chan os.Signal, 1)
		signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
		sig := <-sigChan

		// catches sigint now
		logger.LogAttrs(
			logCtx,
			slog.LevelInfo,
			"server shutdown by an external signal",
			slog.String("signal", sig.String()),
		)

		shutdownCtx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
		defer cancel()

		if err := srv.Shutdown(shutdownCtx); err != nil {
			logger.LogAttrs(
				logCtx,
				slog.LevelError,
				"The HTTP server shutdown error",
				slog.String("error", err.Error()),
			)
		}

		// tell the main thread that this goroutine has finished
		close(idleConnsClosed)
	}()

	err := srv.ListenAndServe()

	if err != http.ErrServerClosed {
		logger.LogAttrs(
			logCtx,
			slog.LevelError,
			"The HTTP server error",
			slog.String("error", err.Error()),
		)
		panic(err)
	}

	<-idleConnsClosed

	return err
}
