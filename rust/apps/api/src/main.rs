use axum::Router;
use axum::routing::{get, post};
pub mod routes;
pub mod schemas;
use routes::auth::{signin_handler};


#[tokio::main]
async fn main(){
    let app = Router::new()
    .nest("/api/v1", 
        Router::new().route("/signup", post(signup))
                            .route("/signin", get(signin_handler)));
    

    let listner = tokio::net::TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listner, app).await.unwrap();

}

async fn signup()-> String{
    format!("signup successfully done!")
}

async fn signin() -> String{
    format!("signin successfully done!")
}