// This starter uses the `axum` crate to create an asyncrohnous web server
// The async runtime being used, is `tokio`
// This starter also has logging, powered by `tracing` and `tracing-subscriber`

use axum::{
    http::{StatusCode, HeaderMap, Method, Uri},
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

// This derive macro allows our main function to run asyncrohnous code. Without it, the main function would run syncrohnously
#[tokio::main]
async fn main() {
    // First, we initialize the tracing subscriber with default configuration
    // This is what allows us to print things to the console
    tracing_subscriber::fmt::init();

    // Then, we create a router, which is a way of routing requests to different handlers
    let app = Router::new()
        // In order to add a route, we use the `route` method on the router
        // The `route` method takes a path (as a &str), and a handler (MethodRouter)
        // In our invocation below, we create a route, that goes to "/"
        // We use `any()` to accept all HTTP methods (GET, POST, PUT, DELETE, etc.)
        // And finally, we provide our route handler
        // The code of the root function is below
        .route("/", any(root))
        // 添加一个通用路由来处理所有其他请求
        .fallback(handle_request);

    // Next, we need to run our app with `hyper`, which is the HTTP server used by `axum`
    // We need to create a `SocketAddr` to run our server on
    // Before we can create that, we need to get the port we wish to serve on
    // This code attempts to get the port from the environment variable `PORT`
    // If it fails to get the port, it will default to "3000"
    // We then parse the `String` into a `u16`, to which if it fails, we panic
    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse()
        .expect("failed to convert to number");

    // We then create a socket address, listening on [::]:PORT (IPv6 binding)
    let ipv6 = SocketAddr::from(([0,0,0,0,0,0,0,0], port));
    // Bind the socket address to a TCP listener which listens for requests
    let ipv6_listener = TcpListener::bind(&ipv6).await.unwrap();

    tracing::info!("Listening on IPv6 at {}!", ipv6);

    // Then, we run the server, using the `serve` method (taking both the TCPListener and axum Router)
    axum::serve(ipv6_listener, app)
    // This function is async, so we need to await it
    .await
    // Then, we unwrap the result, to which if it fails, we panic
    .unwrap();
}
// This is our route handler, for the route root
// Make sure the function is `async`
// We specify our return type, `&'static str`, however a route handler can return anything that implements `IntoResponse`

async fn root(method: Method, uri: Uri, headers: HeaderMap, body: String) -> &'static str {
    print_request_info(&method, &uri, &headers, &body).await;
    "Hello, World!"
}

// 通用请求处理函数，用于处理所有其他路由
async fn handle_request(method: Method, uri: Uri, headers: HeaderMap, body: String) -> impl IntoResponse {
    print_request_info(&method, &uri, &headers, &body).await;
    
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": "Route not found",
            "path": uri.path()
        })),
    )
}

// 打印请求信息的辅助函数
async fn print_request_info(method: &Method, uri: &Uri, headers: &HeaderMap, body: &str) {
    tracing::info!("=== Request Received ===");
    tracing::info!("Method: {}", method);
    tracing::info!("URI: {}", uri);
    tracing::info!("Path: {}", uri.path());
    
    if let Some(query) = uri.query() {
        tracing::info!("Query String: {}", query);
    }
    
    tracing::info!("Headers:");
    for (key, value) in headers.iter() {
        match value.to_str() {
            Ok(v) => tracing::info!("  {}: {}", key, v),
            Err(_) => tracing::info!("  {}: <Binary Data>", key),
        }
    }
    
    tracing::info!("Body:");
    if body.is_empty() {
        tracing::info!("  <Empty>");
    } else {
        tracing::info!("  {}", body);
    }
    
    tracing::info!("=== End of Request Info ===");
}
