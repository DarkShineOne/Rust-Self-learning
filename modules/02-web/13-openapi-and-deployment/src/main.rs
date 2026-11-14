#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind HTTP listener");
    axum::serve(listener, web_13_deployment::app())
        .await
        .expect("run documentation service");
}
