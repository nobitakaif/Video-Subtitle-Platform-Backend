use axum::{
    Router,
    routing::get
};




#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {"Hello world"}));

    let listner = tokio::net::TcpListener::bind("localhost:8080").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
