#![deny(warnings)]

use std::collections::BTreeMap;
use std::fs;

use axum::{Json, Router, response::Html, routing::get};
use tokio::net::TcpListener;

// The message under the logo. Change it and save the file to see hot reload.
const MESSAGE: &str = "Built, deployed, and tested on Kubernetes.";

// The version of the compiler that built the application. build.rs sets it.
const RUNTIME: &str = concat!("Rust ", env!("RUSTC_VERSION"));

// page serves the page with the Okteto logo, which the binary embeds.
async fn page() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

// hello returns the message, the Rust version, and the pod that answers.
// The name of the pod is its hostname, which Linux exposes as a file.
async fn hello() -> Json<BTreeMap<&'static str, String>> {
    let pod = fs::read_to_string("/proc/sys/kernel/hostname").unwrap_or_default();
    Json(BTreeMap::from([
        ("message", MESSAGE.to_string()),
        ("runtime", RUNTIME.to_string()),
        ("pod", pod.trim().to_string()),
    ]))
}

fn app() -> Router {
    Router::new()
        .route("/", get(page))
        .route("/api/info", get(hello))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Starting hello-world server on {}...", listener.local_addr()?);
    axum::serve(listener, app()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    // Sends a request to the router without opening a network connection.
    async fn request(uri: &str) -> (StatusCode, Vec<u8>) {
        let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
        let response = app().oneshot(request).await.unwrap();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, body.to_vec())
    }

    #[tokio::test]
    async fn hello_returns_the_info() {
        let (status, body) = request("/api/info").await;
        assert_eq!(status, StatusCode::OK);

        let Json(info) = Json::<BTreeMap<String, String>>::from_bytes(&body).unwrap();
        assert_eq!(info["message"], MESSAGE);
        assert!(info["runtime"].starts_with("Rust "));
    }

    #[tokio::test]
    async fn page_shows_the_message() {
        let (status, body) = request("/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(String::from_utf8(body).unwrap().contains(r#"id="message""#));
    }
}
