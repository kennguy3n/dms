package server

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"dms/backend-go/internal/config"
)

type fakeDB struct{ err error }

func (f fakeDB) PingContext(context.Context) error { return f.err }

func testServer(t *testing.T) http.Handler {
	t.Helper()
	cfg := config.Config{JWTSecret: "test-secret"}
	logger := slog.New(slog.NewTextHandler(os.Stdout, nil))
	return New(cfg, logger, fakeDB{})
}

func TestHealthz(t *testing.T) {
	srv := testServer(t)
	req := httptest.NewRequest(http.MethodGet, "/healthz", nil)
	rr := httptest.NewRecorder()
	srv.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200 got %d", rr.Code)
	}
}

func TestAuthTokenMockedWorking(t *testing.T) {
	srv := testServer(t)
	body := strings.NewReader("grant_type=password&username=alice&password=secret")
	req := httptest.NewRequest(http.MethodPost, "/v1/auth/token", body)
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	rr := httptest.NewRecorder()
	srv.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200 got %d body=%s", rr.Code, rr.Body.String())
	}

	var resp map[string]any
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("invalid json: %v", err)
	}
	for _, key := range []string{"access_token", "token_type", "expires_in"} {
		if _, ok := resp[key]; !ok {
			t.Fatalf("missing key %s", key)
		}
	}
}
