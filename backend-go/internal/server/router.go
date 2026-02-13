package server

import (
	"context"
	"log/slog"
	"net/http"
	"strings"

	"dms/backend-go/internal/auth"
	"dms/backend-go/internal/config"
	"dms/backend-go/internal/handlers"
	"dms/backend-go/internal/middleware"
)

type pinger interface {
	PingContext(ctx context.Context) error
}

func New(cfg config.Config, logger *slog.Logger, database pinger) http.Handler {
	health := handlers.NewHealthHandler(database)
	validator := auth.NewJWTValidator(cfg.JWTSecret)

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		switch {
		case r.Method == http.MethodGet && r.URL.Path == "/healthz":
			health.Liveness(w, r)
			return
		case r.Method == http.MethodGet && r.URL.Path == "/readyz":
			health.Readiness(w, r)
			return
		case r.Method == http.MethodGet && isTenantWhoAmIPath(r.URL.Path):
			handler := middleware.Authn(logger, validator)(middleware.TenantScope(http.HandlerFunc(handlers.TenantWhoAmI)))
			handler.ServeHTTP(w, r)
			return
		default:
			http.NotFound(w, r)
		}
	})
}

func isTenantWhoAmIPath(path string) bool {
	trimmed := strings.Trim(path, "/")
	parts := strings.Split(trimmed, "/")
	return len(parts) == 4 && parts[0] == "v1" && parts[1] == "tenants" && parts[3] == "whoami" && parts[2] != ""
}
