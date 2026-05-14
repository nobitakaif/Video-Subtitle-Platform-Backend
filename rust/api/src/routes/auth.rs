
mod Schemas;
use Schemas::request_schema;
use axum::handler;

#[handler]
pub fn signin() -> String{
   format!("hello world") 
}