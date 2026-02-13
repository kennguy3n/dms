package middleware

import (
	"net/http"
	"strings"

	"dms/backend-go/internal/model"
)

func TenantScope(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		tenantID := tenantFromPath(r.URL.Path)
		if tenantID == "" {
			http.Error(w, "missing tenantId path parameter", http.StatusBadRequest)
			return
		}

		claims, ok := ClaimsFromContext[*model.Claims](r.Context())
		if !ok {
			http.Error(w, "missing auth context", http.StatusUnauthorized)
			return
		}
		if claims.TenantID != tenantID {
			http.Error(w, "tenant scope mismatch", http.StatusForbidden)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func tenantFromPath(path string) string {
	parts := strings.Split(strings.Trim(path, "/"), "/")
	for i := 0; i < len(parts)-1; i++ {
		if parts[i] == "tenants" {
			return parts[i+1]
		}
	}
	return ""
}
