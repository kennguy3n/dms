#[derive(Debug, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub parent_id: Option<String>,
    pub kind: String,
    pub encrypted_name: String,
    pub mime_type: Option<String>,
    pub size_bytes: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct UploadUrlResponse {
    pub upload_id: String,
    pub put_url: String,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: String,
}
