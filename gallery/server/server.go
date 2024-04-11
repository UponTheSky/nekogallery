package server

import (
	"context"
	"gallery/server/middleware"
	"log/slog"
	"net"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"
)

func New(config *Config) *http.Server {

	var multiplexer http.Handler = http.NewServeMux()

	// register controllers(routers)
	// TODO: delete this mock router
	// TODO: add controllers with a "global" logger
	http.Handle("GET /", multiplexer)

	// register middlewares
	multiplexer = middleware.NewLogMiddleware(multiplexer, config.LoggerWriter)

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

// graceful shutdown
// see the example: https://pkg.go.dev/net/http#Server.Shutdown
func Run(logCtx context.Context, logger *slog.Logger, srv *http.Server) error {
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
