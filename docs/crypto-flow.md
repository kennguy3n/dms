# Reference Crypto Flow Diagram

## Upload and Share Flow (Zero-Knowledge)

```mermaid
sequenceDiagram
    autonumber
    participant U as User Client (Web/Desktop SDK)
    participant K as Local Key Store
    participant A as API
    participant S as Object Storage
    participant DB as Metadata DB

    U->>K: Load user keypair (encrypted private key)
    U->>U: Derive session key from passphrase/device secret
    U->>K: Decrypt private key locally

    U->>U: Generate random DEK per file
    U->>U: Encrypt file bytes with DEK (AEAD)
    U->>U: Encrypt filename/metadata (optional)

    U->>A: Request upload URL (ciphertext metadata only)
    A-->>U: Pre-signed PUT URL + upload_id
    U->>S: PUT encrypted blob
    S-->>U: 200 OK

    U->>U: Wrap DEK to recipients (user/group public keys)
    U->>A: Finalize upload (upload_id + encrypted metadata + wrapped DEKs)
    A->>DB: Persist node/version/envelopes/audit
    A-->>U: File version id

    Note over U,A: Later download
    U->>A: Request download URL for file_version
    A->>DB: Verify ACL + fetch matching DEK envelope
    A-->>U: Pre-signed GET URL + encrypted DEK envelope
    U->>S: GET encrypted blob
    S-->>U: Encrypted bytes
    U->>K: Unwrap DEK with user's private key (local)
    U->>U: Decrypt file locally
```

## Implementation Notes

- Use authenticated encryption only (AES-256-GCM or XChaCha20-Poly1305).
- Generate unique nonce/IV per encryption operation.
- Bind tenant + file identifiers as AEAD associated data.
- Never send plaintext DEK, private keys, or file bytes to API.
- Envelope recipients should include owner and any active share principals.
- Re-key by generating a new DEK and version when revoking broad access.
