use crate::error::{SdkError, SdkResult};

#[derive(Debug, Clone)]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
}

pub trait AuthTokenStore: Send + Sync {
    fn get(&self) -> Option<AuthToken>;
    fn set(&mut self, token: AuthToken);
}

#[derive(Debug, Clone)]
pub struct PasswordGrantRequest {
    pub username: String,
    pub password: String,
}

impl PasswordGrantRequest {
    pub fn to_form_encoded(&self) -> String {
        format!(
            "grant_type=password&username={}&password={}",
            url_encode(&self.username),
            url_encode(&self.password)
        )
    }
}

pub fn bearer_header(token: &AuthToken) -> SdkResult<String> {
    if token.access_token.trim().is_empty() {
        return Err(SdkError::Auth("empty access token".into()));
    }

    let token_type = if token.token_type.trim().is_empty() {
        "Bearer"
    } else {
        token.token_type.trim()
    };

    Ok(format!("{} {}", token_type, token.access_token))
}

fn url_encode(input: &str) -> String {
    let mut out = String::new();
    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}
