use std::sync::Arc;
use reqwest::{Client, Request, Response, header::HeaderValue};
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result};
use http::Extensions;

pub struct AuthHeaderMiddleware {
    pub token_fn: Arc<dyn Fn() -> Option<String> + Send + Sync>,
}

#[async_trait::async_trait]
impl Middleware for AuthHeaderMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let token = (self.token_fn)();
        if let Some(token) = token {
            req.headers_mut().insert(
                "x-annotator-auth-token",
                HeaderValue::from_str(&token).unwrap(),
            );
        }
        next.run(req, extensions).await
    }
}