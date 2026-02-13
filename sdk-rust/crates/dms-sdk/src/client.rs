use std::collections::BTreeMap;

use crate::auth::{bearer_header, AuthToken};
use crate::error::{SdkError, SdkResult};
use crate::models::{Node, TokenResponse, UploadUrlResponse};
use crate::transport::{HttpRequest, HttpTransport};

pub struct DmsClient<T: HttpTransport> {
    base_url: String,
    transport: T,
    token: Option<AuthToken>,
}

impl<T: HttpTransport> DmsClient<T> {
    pub fn new(base_url: impl Into<String>, transport: T) -> Self {
        Self {
            base_url: base_url.into(),
            transport,
            token: None,
        }
    }

    pub fn set_token(&mut self, token: AuthToken) {
        self.token = Some(token);
    }

    pub fn build_token_request(
        &self,
        grant_type: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> SdkResult<HttpRequest> {
        if grant_type.trim().is_empty() {
            return Err(SdkError::InvalidInput("grant_type is required".into()));
        }

        let mut body = format!("grant_type={}", grant_type);
        if let Some(user) = username {
            body.push_str("&username=");
            body.push_str(user);
        }
        if let Some(pass) = password {
            body.push_str("&password=");
            body.push_str(pass);
        }

        let mut headers = BTreeMap::new();
        headers.insert(
            "content-type".into(),
            "application/x-www-form-urlencoded".into(),
        );

        Ok(HttpRequest {
            method: "POST".into(),
            path: format!("{}/auth/token", self.base_url),
            headers,
            body: body.into_bytes(),
        })
    }

    pub fn exchange_token(
        &self,
        grant_type: &str,
        username: Option<&str>,
        password: Option<&str>,
    ) -> SdkResult<TokenResponse> {
        let req = self.build_token_request(grant_type, username, password)?;
        let res = self.transport.send(req)?;
        if res.status >= 400 {
            return Err(SdkError::Api {
                status: res.status,
                message: String::from_utf8_lossy(&res.body).to_string(),
            });
        }

        // Minimal scaffolding: parsing intentionally omitted for now.
        Ok(TokenResponse {
            access_token: String::new(),
            token_type: "Bearer".into(),
            expires_in: 0,
            refresh_token: None,
        })
    }

    pub fn build_list_nodes_request(
        &self,
        tenant_id: &str,
        parent_id: &str,
    ) -> SdkResult<HttpRequest> {
        self.auth_header()?;
        let mut headers = BTreeMap::new();
        headers.insert("authorization".into(), self.auth_header()?);
        Ok(HttpRequest {
            method: "GET".into(),
            path: format!(
                "{}/tenants/{}/nodes?parent_id={}",
                self.base_url, tenant_id, parent_id
            ),
            headers,
            body: Vec::new(),
        })
    }

    pub fn list_nodes(&self, tenant_id: &str, parent_id: &str) -> SdkResult<Vec<Node>> {
        let req = self.build_list_nodes_request(tenant_id, parent_id)?;
        let res = self.transport.send(req)?;
        if res.status >= 400 {
            return Err(SdkError::Api {
                status: res.status,
                message: String::from_utf8_lossy(&res.body).to_string(),
            });
        }
        Ok(vec![])
    }

    pub fn build_upload_url_request(
        &self,
        tenant_id: &str,
        encrypted_name: &str,
        parent_id: &str,
    ) -> SdkResult<HttpRequest> {
        let mut headers = BTreeMap::new();
        headers.insert("authorization".into(), self.auth_header()?);
        headers.insert("content-type".into(), "application/json".into());

        let body = format!(
            "{{\"parent_id\":\"{}\",\"encrypted_name\":\"{}\",\"size_bytes\":0,\"content_hash\":\"\",\"cipher_alg\":\"xchacha20-poly1305\"}}",
            parent_id, encrypted_name
        );

        Ok(HttpRequest {
            method: "POST".into(),
            path: format!("{}/tenants/{}/files/upload-url", self.base_url, tenant_id),
            headers,
            body: body.into_bytes(),
        })
    }

    pub fn create_upload_url(
        &self,
        tenant_id: &str,
        encrypted_name: &str,
        parent_id: &str,
    ) -> SdkResult<UploadUrlResponse> {
        let req = self.build_upload_url_request(tenant_id, encrypted_name, parent_id)?;
        let res = self.transport.send(req)?;
        if res.status >= 400 {
            return Err(SdkError::Api {
                status: res.status,
                message: String::from_utf8_lossy(&res.body).to_string(),
            });
        }
        Ok(UploadUrlResponse {
            upload_id: String::new(),
            put_url: String::new(),
            expires_at: String::new(),
        })
    }

    fn auth_header(&self) -> SdkResult<String> {
        let token = self
            .token
            .as_ref()
            .ok_or_else(|| SdkError::Auth("missing access token".into()))?;
        bearer_header(token)
    }
}
