use dms_sdk::{AuthToken, DmsClient, HttpRequest, HttpResponse, HttpTransport, SdkError, SdkResult};

struct NoopTransport;

impl HttpTransport for NoopTransport {
    fn send(&self, req: HttpRequest) -> SdkResult<HttpResponse> {
        if req.path.contains("/auth/token") {
            return Ok(HttpResponse {
                status: 200,
                headers: Default::default(),
                body: b"{}".to_vec(),
            });
        }

        Err(SdkError::Transport("Noop transport cannot call real API".into()))
    }
}

fn main() {
    let mut client = DmsClient::new("https://api.example.com/v1", NoopTransport);
    client.set_token(AuthToken {
        access_token: "example".into(),
        token_type: "Bearer".into(),
        expires_in: 3600,
        refresh_token: None,
    });

    let _ = client.build_list_nodes_request("tenant-123", "root-node");
}
