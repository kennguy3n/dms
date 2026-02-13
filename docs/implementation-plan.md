# Implementation Plan

This plan turns the architecture artifacts into an executable delivery roadmap with **Go backend services** and **Rust SDKs** for headless integration.

## 0) Technology Decisions (Locked)

- **Backend language:** Go (Go 1.23+).
- **Primary backend framework:** standard `net/http` + `chi` (or `gin`) with strong middleware for tenant/auth enforcement.
- **Data layer:** PostgreSQL for metadata + ACL + audit.
- **Blob storage:** S3-compatible object store for ciphertext objects.
- **Queue/events (optional phase 2):** NATS or Kafka.
- **SDK language:** Rust (first-party) with optional FFI bindings for desktop ecosystems later.
- **Web crypto:** Browser-side WebCrypto (or WASM via Rust crypto core in phase 3).

## 1) Repository & Service Layout

```text
/dms
  /api
    openapi.yaml
  /docs
    data-model.md
    crypto-flow.md
    implementation-plan.md
  /backend-go
    /cmd/api
    /internal/auth
    /internal/tenant
    /internal/nodes
    /internal/files
    /internal/shares
    /internal/audit
    /internal/storage
    /internal/db
    /internal/middleware
    /migrations
  /sdk-rust
    /crates/dms-sdk
    /crates/dms-crypto
    /examples
```

## 2) Delivery Phases

### Phase 1 — Foundations (Week 1-2)

**Backend (Go)**
- Scaffold API service, config, structured logging, health/readiness endpoints.
- Add auth middleware (JWT bearer) and request context extraction.
- Add tenant-scope middleware that validates `tenantId` path + token tenant claims.
- Set up PostgreSQL migrations for core entities from ERD.

**SDK (Rust)**
- Create `dms-sdk` crate with typed API client scaffolding.
- Create `dms-crypto` crate for DEK generation, envelope wrapping/unwrapping interfaces.
- Add HTTP auth/token helpers and typed error model.

**Exit criteria**
- `POST /auth/token` mocked/working.
- Migrations run cleanly in CI/local.
- Rust SDK compiles and can call health/auth endpoints.

### Phase 2 — File Upload/Download Core (Week 3-5)

**Backend (Go)**
- Implement `POST /tenants/{tenantId}/files/upload-url`.
- Implement `POST /tenants/{tenantId}/files/finalize`.
- Persist `node`, `file_object`, `file_version`, `file_key_envelope`, `audit_log`.
- Implement `POST /tenants/{tenantId}/files/{fileVersionId}/download-url` with ACL checks.
- S3 pre-signed URL support for PUT/GET.

**SDK (Rust)**
- Implement encrypted upload flow:
  1. generate DEK,
  2. encrypt file stream locally,
  3. call upload-url,
  4. upload ciphertext,
  5. finalize with wrapped DEKs.
- Implement download/decrypt flow:
  1. request download-url + envelope,
  2. fetch ciphertext,
  3. unwrap DEK locally,
  4. decrypt bytes.

**Exit criteria**
- End-to-end upload/download works with ciphertext-only server storage.
- No plaintext file/key material logged or persisted server-side.

### Phase 3 — Hierarchy, Sharing, and Audit (Week 6-8)

**Backend (Go)**
- Implement `GET /tenants/{tenantId}/nodes`.
- Implement `POST /tenants/{tenantId}/shares`.
- Implement `GET /tenants/{tenantId}/audit` with cursor pagination.
- Add ACL inheritance resolution and policy tests.

**SDK (Rust)**
- Add high-level share-link creation APIs.
- Add node browsing and pagination helpers.
- Add audit query client methods for admin tooling.

**Exit criteria**
- Internal sharing + basic external share link flows functional.
- Tenant audit trail queryable and stable.

### Phase 4 — Hardening & Enterprise Controls (Week 9-10)

**Backend (Go)**
- Key rotation and envelope revocation mechanics.
- Rate limits + abuse controls + tamper-evident audit signatures.
- SSO hooks (OIDC/SAML gateway integration).

**SDK (Rust)**
- Retry/backoff policies, resumable transfer helpers.
- Optional secure local key cache abstraction for desktop apps.

**Exit criteria**
- Security review checklist pass.
- Load test target met.

## 3) API-to-Implementation Mapping

- `/auth/token` → `internal/auth`
- `/files/upload-url`, `/files/finalize`, `/files/{id}/download-url` → `internal/files` + `internal/storage`
- `/nodes` → `internal/nodes`
- `/shares` → `internal/shares`
- `/audit` → `internal/audit`

All handlers must call shared middleware:
1. authn
2. tenant scope validation
3. role/ACL authorization
4. audit emission

## 4) Testing Strategy

### Go backend
- Unit tests for ACL evaluator, tenant filters, request validators.
- Integration tests with ephemeral Postgres + S3 mock (e.g., MinIO).
- Contract tests generated from OpenAPI examples.

### Rust SDK
- Unit tests for crypto envelope and parsing.
- Integration tests against local Go API test environment.
- Property tests for encrypt/decrypt round-trip invariants.

### Security tests
- Ensure no endpoint returns plaintext keys.
- Verify tenant escape attempts are rejected.
- Verify revoked envelope cannot decrypt new versions.

## 5) CI/CD Gates

- Go: `go fmt`, `go vet`, `go test ./...`.
- Rust: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`.
- OpenAPI lint + drift check against implemented routes.
- Migration up/down validation in CI.

## 6) Risks & Mitigations

- **Risk:** accidental plaintext metadata leakage.
  - **Mitigation:** explicit encrypted fields, redaction middleware, logging policy tests.
- **Risk:** envelope/key lifecycle complexity.
  - **Mitigation:** central key-envelope service in Go + reusable Rust crypto primitives.
- **Risk:** multi-tenant authorization bugs.
  - **Mitigation:** mandatory tenant predicates in repositories + policy tests per endpoint.

## 7) Immediate Next Tasks (Sprint-ready)

1. Scaffold `backend-go` service with middleware skeleton.
2. Create initial SQL migrations from `docs/data-model.md`.
3. Generate Go server stubs and Rust client stubs from `api/openapi.yaml`.
4. Implement upload-url + finalize vertical slice end-to-end.
5. Add first Rust SDK encrypted upload example.
