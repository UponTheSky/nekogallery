package logger

import (
	"io"
	"log"
	"log/slog"
)

// The logger handling http.Server relevant errors.
//
// This will be assigned to http.Server.ErrorLog, so the type is *log.Logger.
func NewServerErrorLogger(w io.Writer, level slog.Level) *log.Logger {
	return slog.NewLogLogger(slog.NewTextHandler(w, nil), level)
}

// The logger handling general structured logs
func NewSLogger(w io.Writer, level slog.Level) *slog.Logger {
	return slog.New(slog.NewTextHandler(w, &slog.HandlerOptions{Level: level}))
}
