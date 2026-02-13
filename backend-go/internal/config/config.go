package config

import (
	"log/slog"
	"os"
)

type Config struct {
	HTTPAddr    string
	DatabaseURL string
	JWTSecret   string
	LogLevel    slog.Level
}

func Load() Config {
	return Config{
		HTTPAddr:    envOrDefault("HTTP_ADDR", ":8080"),
		DatabaseURL: envOrDefault("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/dms?sslmode=disable"),
		JWTSecret:   envOrDefault("JWT_SECRET", "change-me"),
		LogLevel:    parseLogLevel(envOrDefault("LOG_LEVEL", "INFO")),
	}
}

func envOrDefault(key, fallback string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return fallback
}

func parseLogLevel(raw string) slog.Level {
	switch raw {
	case "DEBUG":
		return slog.LevelDebug
	case "WARN":
		return slog.LevelWarn
	case "ERROR":
		return slog.LevelError
	default:
		return slog.LevelInfo
	}
}
