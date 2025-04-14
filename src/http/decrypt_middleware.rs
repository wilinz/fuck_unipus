use std::sync::Arc;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use http::Extensions;
use serde_json::Value;
use crate::core::decrypt::decrypt_unipus_content;
use anyhow::{self, Error};

// 判断是否是 Unipus 加密的响应
fn is_unipus_encrypted_response(url: &str, content_type: Option<&str>) -> bool {
    url.starts_with("https://ucontent.unipus.cn/course/api/v3/content/course-v1")
        && content_type.map_or(false, |ct| ct.starts_with("application/json"))
}

// 尝试解密 "content" 字段
fn try_decrypt_content(json: &mut Value) -> Option<String> {
    let content_str = json.get("content").and_then(|v| v.as_str())?;
    let k_str = json.get("k").and_then(|v| v.as_str())?;
    Some(decrypt_unipus_content(content_str, k_str).unwrap_or("".to_string()))
}

pub struct DecryptMiddleware;

#[async_trait::async_trait]
impl Middleware for DecryptMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        // Perform the request and get the response
        let mut response = next.run(req, extensions).await?;
        let url_str = response.url().to_string();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok());

        // 判断是否需要解密
        if !is_unipus_encrypted_response(&url_str, content_type) {
            return Ok(response); // 不需要解密，直接返回
        }

        let status = response.status();
        let version = response.version();
        let headers = response.headers().clone();
        let extensions_clone = response.extensions().clone();

        // 获取响应体
        let body_bytes = response.bytes().await?;
        let mut json: Value = serde_json::from_slice(&body_bytes)
            .map_err(|_| reqwest_middleware::Error::Middleware(anyhow::anyhow!("JSON 解析失败").into()))?;

        let mut new_body = body_bytes.to_vec();
        // 尝试解密
        if let Some(decrypted_content) = try_decrypt_content(&mut json) {
            // remove unicode encode. e.g. \u0001
            let json: Value = serde_json::from_str(&decrypted_content).unwrap();
            new_body = serde_json::to_vec(&json).unwrap();
            // if let Some(obj) = json.as_object_mut() {
            //     obj.insert("content".to_string(), Value::String(decrypted_content));
            // }
        } else {
            eprintln!("❌ 解密失败：无法解析 content 或密钥");
        }

        // 重建响应体


        // 构建新的 http::Response
        let mut builder = http::Response::builder()
            .status(status)
            .version(version);

        let headers_map = builder.headers_mut().unwrap();
        for (key, value) in headers.iter() {
            headers_map.insert(key, value.clone());
        }

        let http_response = builder
            .body(new_body)
            .expect("构造响应失败");

        let mut final_response = Response::from(http_response);
        final_response.extensions_mut().extend(extensions_clone);

        Ok(final_response)
    }
}