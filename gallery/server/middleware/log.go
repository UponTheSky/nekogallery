package middleware

import (
	"gallery/utils/logger"
	"log/slog"
	"net/http"
	"os"
)

func NewLogMiddleware(handler http.Handler) http.Handler {
	requestLogger := logger.NewSLogger(os.Stdout, slog.LevelDebug)

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// log request
		requestLogger.LogAttrs(
			r.Context(),
			slog.LevelInfo,
			"Request",
			slog.String("method", r.Method),
			slog.String("url", r.URL.String()),
			slog.String("protocol", r.Proto),
			slog.String("host", r.Host),
		)

		handler.ServeHTTP(w, r)
	})
}
