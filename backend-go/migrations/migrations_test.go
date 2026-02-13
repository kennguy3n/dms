package migrations

import (
	"os"
	"strings"
	"testing"
)

func TestCoreMigrationContainsExpectedStatements(t *testing.T) {
	data, err := os.ReadFile("0001_init_core.sql")
	if err != nil {
		t.Fatalf("failed to read migration: %v", err)
	}
	sql := string(data)

	required := []string{
		"CREATE TABLE IF NOT EXISTS tenants",
		"CREATE TABLE IF NOT EXISTS users",
		"CREATE TABLE IF NOT EXISTS nodes",
		"CREATE TABLE IF NOT EXISTS file_versions",
		"CREATE TABLE IF NOT EXISTS audit_logs",
	}
	for _, stmt := range required {
		if !strings.Contains(sql, stmt) {
			t.Fatalf("migration missing statement: %s", stmt)
		}
	}

	if strings.Count(sql, ";") < 15 {
		t.Fatalf("migration appears incomplete; expected at least 15 statements")
	}
}
