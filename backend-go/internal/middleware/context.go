package middleware

import "context"

type contextKey string

const claimsContextKey contextKey = "claims"

func WithClaims(ctx context.Context, claims any) context.Context {
	return context.WithValue(ctx, claimsContextKey, claims)
}

func ClaimsFromContext[T any](ctx context.Context) (T, bool) {
	claims, ok := ctx.Value(claimsContextKey).(T)
	return claims, ok
}
