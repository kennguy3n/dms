package model

type Claims struct {
	UserID   string `json:"sub"`
	TenantID string `json:"tenant_id"`
	Email    string `json:"email,omitempty"`
	Exp      int64  `json:"exp,omitempty"`
}
