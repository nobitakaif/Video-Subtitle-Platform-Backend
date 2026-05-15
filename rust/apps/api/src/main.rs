use axum::Router;
use axum::routing::{get, post};
pub mod routes;
pub mod schemas;

use routes::auth::{signin_handler};

use crate::routes::auth::signup_handler;
use store::store::Store;
#[tokio::main]
async fn main(){
    let app = Router::new()
    .nest("/api/v1", 
        Router::new().route("/signup", post(signup_handler))
                            .route("/signin", post(signin_handler)));
    let listner = tokio::net::TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listner, app).await.unwrap();

}
