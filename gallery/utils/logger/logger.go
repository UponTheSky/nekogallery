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
	return nil
}

// The logger handling incoming *http.Request instances.
func NewRequestLogger() {

}

// The logger handling outgoing *http.Response instances.
func NewResponseLogger() {

}

// The logger handling internal service events.
func NewSystemLogger() {

}
