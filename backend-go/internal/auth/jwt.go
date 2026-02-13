package auth

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"errors"
	"strings"
	"time"

	"dms/backend-go/internal/model"
)

type JWTValidator struct {
	secret []byte
}

func NewJWTValidator(secret string) *JWTValidator {
	return &JWTValidator{secret: []byte(secret)}
}

func (v *JWTValidator) Parse(tokenString string) (*model.Claims, error) {
	parts := strings.Split(tokenString, ".")
	if len(parts) != 3 {
		return nil, errors.New("invalid token format")
	}

	sigCheck := sign(parts[0]+"."+parts[1], v.secret)
	if !hmac.Equal([]byte(parts[2]), []byte(sigCheck)) {
		return nil, errors.New("invalid token signature")
	}

	payload, err := base64.RawURLEncoding.DecodeString(parts[1])
	if err != nil {
		return nil, err
	}

	var claims model.Claims
	if err := json.Unmarshal(payload, &claims); err != nil {
		return nil, err
	}
	if claims.TenantID == "" || claims.UserID == "" {
		return nil, errors.New("token missing required claims")
	}
	if claims.Exp != 0 && time.Now().Unix() > claims.Exp {
		return nil, errors.New("token expired")
	}
	return &claims, nil
}

func sign(msg string, secret []byte) string {
	h := hmac.New(sha256.New, secret)
	h.Write([]byte(msg))
	return base64.RawURLEncoding.EncodeToString(h.Sum(nil))
}
