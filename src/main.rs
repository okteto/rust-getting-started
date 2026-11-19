#![deny(warnings)]

use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello world!"
}

fn app() -> Router {
    Router::new().route("/", get(hello))
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

    #[tokio::test]
    async fn hello_returns_hello_world() {
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = app().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body, "Hello world!");
    }
}
