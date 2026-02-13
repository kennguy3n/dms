package handlers

import (
	"net/http"

	"dms/backend-go/internal/middleware"
	"dms/backend-go/internal/model"
)

func TenantWhoAmI(w http.ResponseWriter, r *http.Request) {
	claims, _ := middleware.ClaimsFromContext[*model.Claims](r.Context())
	writeJSON(w, http.StatusOK, map[string]string{
		"user_id":   claims.UserID,
		"tenant_id": claims.TenantID,
	})
}
