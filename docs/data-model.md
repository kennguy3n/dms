# ERD and Data Model (Multi-Tenant Zero-Knowledge DMS)

## ERD

```mermaid
erDiagram
    TENANT ||--o{ USER : has
    TENANT ||--o{ GROUP : has
    TENANT ||--o{ PROJECT : has
    TENANT ||--o{ FILE_OBJECT : stores
    TENANT ||--o{ AUDIT_LOG : records

    USER ||--o{ USER_KEYPAIR : owns
    USER ||--o{ API_TOKEN : creates
    USER ||--o{ SHARE_LINK : creates

    GROUP ||--o{ GROUP_MEMBER : includes
    USER ||--o{ GROUP_MEMBER : joins

    PROJECT ||--o{ NODE : contains
    NODE ||--o{ NODE : parent_of
    NODE ||--o{ FILE_VERSION : has

    FILE_OBJECT ||--o{ FILE_VERSION : backs
    FILE_VERSION ||--o{ FILE_KEY_ENVELOPE : grants
    USER ||--o{ FILE_KEY_ENVELOPE : receives
    GROUP ||--o{ FILE_KEY_ENVELOPE : receives

    NODE ||--o{ ACL_ENTRY : protected_by
    USER ||--o{ ACL_ENTRY : assigned
    GROUP ||--o{ ACL_ENTRY : assigned

    NODE ||--o{ SHARE_LINK : shared_as

    TENANT {
      uuid id PK
      string name
      string slug UK
      string status
      datetime created_at
      datetime updated_at
    }

    USER {
      uuid id PK
      uuid tenant_id FK
      string email UK
      string display_name
      string status
      datetime last_login_at
      datetime created_at
    }

    USER_KEYPAIR {
      uuid id PK
      uuid user_id FK
      string algorithm
      text public_key
      text encrypted_private_key
      string key_version
      datetime created_at
      datetime revoked_at
    }

    GROUP {
      uuid id PK
      uuid tenant_id FK
      string name
      datetime created_at
    }

    GROUP_MEMBER {
      uuid group_id FK
      uuid user_id FK
      string role
      datetime created_at
    }

    PROJECT {
      uuid id PK
      uuid tenant_id FK
      string name
      uuid root_node_id FK
      datetime created_at
    }

    NODE {
      uuid id PK
      uuid tenant_id FK
      uuid project_id FK
      uuid parent_id FK
      string kind
      text encrypted_name
      string mime_type
      int64 size_bytes
      string status
      uuid created_by FK
      datetime created_at
      datetime updated_at
      datetime deleted_at
    }

    FILE_OBJECT {
      uuid id PK
      uuid tenant_id FK
      string storage_provider
      string storage_key
      string content_hash
      int64 size_bytes
      datetime created_at
    }

    FILE_VERSION {
      uuid id PK
      uuid tenant_id FK
      uuid node_id FK
      uuid file_object_id FK
      int version_no
      string cipher_alg
      text encrypted_metadata
      uuid created_by FK
      datetime created_at
    }

    FILE_KEY_ENVELOPE {
      uuid id PK
      uuid tenant_id FK
      uuid file_version_id FK
      string principal_type
      uuid principal_id
      text encrypted_dek
      string key_wrap_alg
      datetime created_at
      datetime revoked_at
    }

    ACL_ENTRY {
      uuid id PK
      uuid tenant_id FK
      uuid node_id FK
      string principal_type
      uuid principal_id
      string permission
      bool inherited
      datetime created_at
    }

    SHARE_LINK {
      uuid id PK
      uuid tenant_id FK
      uuid node_id FK
      uuid created_by FK
      text token_hash
      datetime expires_at
      int max_downloads
      int used_downloads
      bool password_required
      string permission
      datetime created_at
      datetime revoked_at
    }

    API_TOKEN {
      uuid id PK
      uuid tenant_id FK
      uuid user_id FK
      text token_hash
      string scope
      datetime expires_at
      datetime last_used_at
      datetime created_at
      datetime revoked_at
    }

    AUDIT_LOG {
      uuid id PK
      uuid tenant_id FK
      uuid actor_user_id FK
      string action
      string target_type
      uuid target_id
      json metadata
      string source_ip
      datetime occurred_at
      text signature
    }
```

## Notes

- `NODE.kind` is `folder` or `file`; only `file` nodes have versions.
- `FILE_OBJECT` is immutable blob storage metadata; multiple versions can deduplicate by `content_hash`.
- `FILE_KEY_ENVELOPE` supports user- and group-based key grants.
- `encrypted_name` enables encrypted filename support with server-side opaque metadata.
- Tenant isolation should be enforced in every query by `tenant_id`.
