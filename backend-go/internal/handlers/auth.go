package handlers

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"strings"
	"time"
)

type AuthHandler struct {
	secret string
}

func NewAuthHandler(secret string) *AuthHandler {
	return &AuthHandler{secret: secret}
}

type tokenRequest struct {
	GrantType string `json:"grant_type"`
	Username  string `json:"username"`
	Password  string `json:"password"`
}

func (h *AuthHandler) Token(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	req := tokenRequest{}
	ct := r.Header.Get("Content-Type")
	if strings.Contains(ct, "application/json") {
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "invalid json body", http.StatusBadRequest)
			return
		}
	} else {
		if err := r.ParseForm(); err != nil {
			http.Error(w, "invalid form body", http.StatusBadRequest)
			return
		}
		req.GrantType = r.FormValue("grant_type")
		req.Username = r.FormValue("username")
		req.Password = r.FormValue("password")
	}

	if req.GrantType == "" {
		http.Error(w, "grant_type is required", http.StatusBadRequest)
		return
	}

	userID := "mock-user"
	tenantID := "mock-tenant"
	if req.Username != "" {
		userID = req.Username
	}
	token := mintHS256Token(h.secret, userID, tenantID, time.Now().Add(1*time.Hour).Unix())

	writeJSON(w, http.StatusOK, map[string]any{
		"access_token":  token,
		"token_type":    "Bearer",
		"expires_in":    3600,
		"refresh_token": "mock-refresh-token",
	})
}

func mintHS256Token(secret, subject, tenantID string, exp int64) string {
	header := base64.RawURLEncoding.EncodeToString([]byte(`{"alg":"HS256","typ":"JWT"}`))
	payloadBytes, _ := json.Marshal(map[string]any{
		"sub":       subject,
		"tenant_id": tenantID,
		"exp":       exp,
	})
	payload := base64.RawURLEncoding.EncodeToString(payloadBytes)
	unsigned := header + "." + payload
	sig := signJWT(unsigned, []byte(secret))
	return unsigned + "." + sig
}

func signJWT(msg string, secret []byte) string {
	h := hmac.New(sha256.New, secret)
	h.Write([]byte(msg))
	return base64.RawURLEncoding.EncodeToString(h.Sum(nil))
}
