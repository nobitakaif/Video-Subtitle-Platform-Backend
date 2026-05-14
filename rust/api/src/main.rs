use axum::Router;
use axum::routing::{get, post};



#[tokio::main]
async fn main(){
    let app = Router::new()
    .nest("/api/v1", 
        Router::new().route("/signup", get(signup))
                            .route("/signin", get(signin)));
    

    let listner = tokio::net::TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listner, app).await.unwrap();

}

async fn signup()-> String{
    format!("signup successfully done!")
}

async fn signin() -> String{
    format!("signin successfully done!")
}