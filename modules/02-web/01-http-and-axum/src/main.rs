#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("bind local HTTP listener");
    let application = web_01_http::app();
    axum::serve(listener, application)
        .await
        .expect("run HTTP server");
}
